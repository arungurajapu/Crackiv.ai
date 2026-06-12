use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub struct AppState {
    pub keys: RwLock<Vec<String>>,
    pub active_key_index: RwLock<usize>,
    pub screenshots: Arc<Mutex<VecDeque<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(Vec::new()),
            active_key_index: RwLock::new(0),
            screenshots: Arc::new(Mutex::new(VecDeque::with_capacity(10))),
        }
    }
}
