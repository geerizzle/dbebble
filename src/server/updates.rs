use crate::{logger::Logger, parser::update::UpdateParser, statics::API_URL};

use super::{cache::ServerCache, generate_headers};
use reqwest::Client;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time;

pub struct UpdatesFetcher {
    client: Client,
    cache: Arc<Mutex<ServerCache>>,
    logger: Arc<Mutex<Logger>>,
}

impl UpdatesFetcher {
    pub fn new(cache: Arc<Mutex<ServerCache>>, logger: Arc<Mutex<Logger>>) -> Self {
        Self {
            client: Client::default(),
            cache,
            logger,
        }
    }

    pub async fn start(&mut self) {
        let mut interval = time::interval(Duration::from_secs(30));
        let update_parser = UpdateParser::new(Arc::clone(&self.logger), Arc::clone(&self.cache));
        loop {
            let request = self
                .client
                .get(format!(
                    "{}/rchg/{}",
                    API_URL,
                    self.cache.lock().unwrap().get_eva_id()
                ))
                .headers(generate_headers(&self.cache.lock().unwrap()));

            let response = request.send().await.unwrap().text().await.unwrap();
            let parsed = update_parser.parse_update(response.as_str());
            interval.tick().await;
        }
    }
}
