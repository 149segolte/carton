use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use hcloud::apis::configuration::Configuration;
use hcloud::apis::{primary_ips_api, servers_api};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskNames {
    ProviderOverview,
    FetchServers,
    Nop,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: TaskNames,
    pub data: serde_json::Value,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            name: TaskNames::Nop,
            data: serde_json::Value::Null,
        }
    }
}

impl Task {
    pub fn new(name: TaskNames, data: serde_json::Value) -> Self {
        Self { name, data }
    }

    async fn run(&self) -> Option<Task> {
        match self.name {
            TaskNames::ProviderOverview => {
                let data = self.data.as_object().unwrap();
                let _ = data.get("auth").unwrap().as_str().unwrap();
                let token = data.get("token").unwrap().as_str().unwrap();

                let mut overview = json!({
                    "name": "Hetzner Cloud",
                    "status": "Connected",
                    "servers": 0,
                    "primary_ips": 0,
                    "firewalls": 0,
                    "load_balancers": 0,
                });

                let mut configuration = Configuration::new();
                configuration.bearer_access_token = Some(token.to_string());

                let resp = servers_api::list_servers(&configuration, Default::default()).await;
                if resp.is_ok() {
                    overview["servers"] = json!(resp.unwrap().servers.len());
                } else {
                    overview["status"] =
                        format!("Disconnected, Error: {:?}", resp.err().unwrap()).into();
                }

                let resp =
                    primary_ips_api::list_primary_ips(&configuration, Default::default()).await;
                if resp.is_ok() {
                    overview["primary_ips"] = json!(resp.unwrap().primary_ips.len());
                } else {
                    overview["status"] =
                        format!("Disconnected, Error: {:?}", resp.err().unwrap()).into();
                }

                Some(Task::new(TaskNames::ProviderOverview, overview))
            }
            TaskNames::FetchServers => {
                let data = self.data.as_object().unwrap();
                let _ = data.get("auth").unwrap().as_str().unwrap();
                let token = data.get("token").unwrap().as_str().unwrap();

                let mut configuration = Configuration::new();
                configuration.bearer_access_token = Some(token.to_string());

                // get list of all existing servers from servers API
                let servers = servers_api::list_servers(&configuration, Default::default())
                    .await
                    .ok()?
                    .servers;
                Some(Task::new(
                    TaskNames::FetchServers,
                    json!({ "servers": servers }),
                ))
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Runner {
    tx: Option<mpsc::Sender<(Uuid, bool, String)>>,
    data: Arc<Mutex<HashMap<Uuid, (bool, Task)>>>,
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            tx: None,
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Runner {
    pub fn new() -> Self {
        let mut new = Self::default();

        let (tx, rx) = mpsc::channel();
        new.tx = Some(tx);

        let storage = new.data.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                while let Ok((id, update_flag, data)) = rx.recv() {
                    let task: Task = serde_json::from_str(&data).unwrap();
                    let val = task.run().await.unwrap_or(Task::default());
                    let mut store = storage.lock().unwrap();
                    store.insert(id, (update_flag, val));
                }
            })
        });

        new
    }

    pub fn add_async_task(self, update_flag: bool, input: Task) -> Uuid {
        let id = Uuid::new_v4();
        self.tx
            .unwrap()
            .send((id, update_flag, serde_json::to_string(&input).unwrap()))
            .expect("cannot send");
        id
    }

    pub fn get_async_task(self, id: Uuid) -> Option<(bool, Task)> {
        let mut store = self.data.lock().unwrap();
        store.remove(&id)
    }
}
