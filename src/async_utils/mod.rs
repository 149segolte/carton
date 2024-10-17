use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use uuid::Uuid;

#[derive(Clone)]
pub struct Runner {
    tx: Option<mpsc::Sender<(Uuid, bool, String)>>,
    data: Arc<Mutex<HashMap<Uuid, (bool, String)>>>,
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
            while let Ok((id, update_flag, data)) = rx.recv() {
                let mut store = storage.lock().unwrap();
                store.insert(id, (update_flag, data));
            }
        });

        new
    }

    pub fn add_async_task(self, update_flag: bool, input: String) -> Uuid {
        let id = Uuid::new_v4();
        self.tx
            .unwrap()
            .send((id, update_flag, input))
            .expect("cannot send");
        id
    }

    pub fn get_async_data(self, id: Uuid) -> Option<(bool, String)> {
        let mut store = self.data.lock().unwrap();
        store.remove(&id)
    }
}
