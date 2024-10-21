use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use anyhow::Result;
use hcloud::apis::configuration::Configuration;
use hcloud::apis::{primary_ips_api, servers_api};
use tokio::runtime::Runtime;
use tuirealm::listener::{ListenerResult, Poll};
use tuirealm::Event;

use crate::constants::{
    Config, ProviderStatus, ServerHandle, ServerListStatus, UserEvent, UserEventIter,
};

#[derive(Debug, Clone)]
pub enum Tasks {
    ProviderStatus,
    FetchServers,
    Nop,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub request: Tasks,
    pub response: Option<UserEvent>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            request: Tasks::Nop,
            response: None,
        }
    }
}

impl Task {
    pub fn new(request: Tasks) -> Self {
        Self {
            request,
            ..Default::default()
        }
    }

    async fn run(&mut self, config: Config) -> Result<()> {
        match &self.request {
            Tasks::ProviderStatus => {
                let mut overview = ProviderStatus::new(config.auth.auth.clone());

                let mut configuration = Configuration::new();
                configuration.bearer_access_token = Some(config.auth.token.to_string());

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

                self.response = Some(UserEvent::ProviderStatus(overview));
            }
            Tasks::FetchServers => {
                let mut configuration = Configuration::new();
                configuration.bearer_access_token = Some(config.auth.token.to_string());

                let resp = servers_api::list_servers(&configuration, Default::default()).await;
                if resp.is_ok() {
                    self.response = Some(UserEvent::ServerListStatus(ServerListStatus::new(
                        resp.unwrap()
                            .servers
                            .iter()
                            .map(|s| ServerHandle::Hetzner(Box::new(s.clone())))
                            .collect(),
                    )));
                }
            }
            Tasks::Nop => {
                self.response = Some(UserEvent::Empty);
            }
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
    pub fn new(config: Config) -> Self {
        let (tx, rx) = mpsc::channel::<Task>();
        let store = Arc::new(Mutex::new(Vec::new()));

        let inner_store = store.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                while let Ok(mut task) = rx.recv() {
                    if let Err(err) = task.run(config.clone()).await {
                        task.response = Some(UserEvent::Error(err.to_string()));
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
            .filter_map(|task| task.response)
            .collect::<Vec<_>>();

        Ok(Some(Event::User(UserEventIter::new(events))))
    }
}
