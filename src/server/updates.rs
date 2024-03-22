use super::cache::ServerCache;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time;

pub struct UpdatesFetcher {
    cache: Arc<Mutex<ServerCache>>,
}

impl UpdatesFetcher {
    pub fn new(cache: Arc<Mutex<ServerCache>>) -> Self {
        Self { cache }
    }

    pub async fn start(&mut self) {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            println!("5 second task");
            interval.tick().await;
        }
    }
}
