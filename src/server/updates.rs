use super::cache::ServerCache;
use std::time::Duration;
use tokio::time;

pub struct UpdatesFetcher {
    cache: ServerCache,
}

impl UpdatesFetcher {
    pub fn new(cache: ServerCache) -> Self {
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
