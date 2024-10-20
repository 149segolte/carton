use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use anyhow::Result;
use hcloud::apis::configuration::Configuration;
use hcloud::apis::{primary_ips_api, servers_api};
use tokio::runtime::Runtime;
use tuirealm::listener::{ListenerResult, Poll};
use tuirealm::Event;
use uuid::Uuid;

use crate::constants::{Auth, ProviderStatus, UserEvent, UserEventIter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum SingletonTasks {
    ProviderStatus(Auth),
    FetchServers(Auth),
    Nop,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Tasks {
    Nop,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum TaskRequest {
    Single(SingletonTasks),
    Multiple(Uuid, Tasks),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum TaskResponse {
    ProviderStatus(ProviderStatus),
    Error(String),
    Empty,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Task {
    pub request: TaskRequest,
    pub response: Option<TaskResponse>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            request: TaskRequest::Single(SingletonTasks::Nop),
            response: None,
        }
    }
}

impl Task {
    pub fn new(request: TaskRequest) -> Self {
        Self {
            request,
            ..Default::default()
        }
    }

    async fn run(&mut self) -> Result<()> {
        if let TaskRequest::Single(task) = &self.request {
            match task {
                SingletonTasks::ProviderStatus(auth) => {
                    let mut overview = ProviderStatus::new(auth.auth.clone());

                    let mut configuration = Configuration::new();
                    configuration.bearer_access_token = Some(auth.token.to_string());

                    let resp = servers_api::list_servers(&configuration, Default::default()).await;
                    if resp.is_ok() {
                        overview.servers = resp.unwrap().servers.len();
                    } else {
                        overview.status = format!("Disconnected, Error: {:?}", resp.err().unwrap());
                    }

                    let resp =
                        primary_ips_api::list_primary_ips(&configuration, Default::default()).await;
                    if resp.is_ok() {
                        overview.primary_ips = resp.unwrap().primary_ips.len();
                    } else {
                        overview.status = format!("Disconnected, Error: {:?}", resp.err().unwrap());
                    }

                    self.response = Some(TaskResponse::ProviderStatus(overview));
                }
                _ => {
                    self.response = Some(TaskResponse::Empty);
                }
            }
        } else {
            self.response = Some(TaskResponse::Empty);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct TaskHandler {
    tx: Option<mpsc::Sender<Task>>,
    completed: Arc<Mutex<Vec<Task>>>,
}

impl TaskHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Task>();
        let store = Arc::new(Mutex::new(Vec::new()));

        let inner_store = store.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                while let Ok(mut task) = rx.recv() {
                    if let Err(err) = task.run().await {
                        task.response = Some(TaskResponse::Error(err.to_string()));
                    }
                    let mut store = inner_store.lock().unwrap();
                    store.push(task);
                }
            })
        });

        Self {
            tx: Some(tx),
            completed: store,
        }
    }

    pub fn add_task(self, task: Task) {
        self.tx.unwrap().send(task).expect("cannot send");
    }
}

impl Poll<UserEventIter> for TaskHandler {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEventIter>>> {
        let mut completed = self.completed.lock().unwrap();
        if completed.is_empty() {
            return Ok(None);
        }

        let events = completed
            .drain(..)
            .filter_map(|task| match task.response {
                Some(TaskResponse::ProviderStatus(status)) => {
                    Some(UserEvent::ProviderStatus(status))
                }
                Some(TaskResponse::Error(err)) => Some(UserEvent::Error(err)),
                _ => None,
            })
            .collect::<Vec<_>>();

        Ok(Some(Event::User(UserEventIter::new(events))))
    }
}
