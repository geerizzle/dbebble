use std::collections::{BTreeMap, HashMap};

use crate::{env::APIKeys, parser::eva::EvaParser, statics::API_URL};

use super::generate_headers;

#[derive(Default)]
pub(crate) struct ServerCache {
    current_plan: BTreeMap<String, String>,
    eva_map: HashMap<String, String>,
    pub(crate) creds: APIKeys,
    eva_id: String,
    from: String,
    to: String,
}

impl ServerCache {
    pub(crate) fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            ..Self::default()
        }
    }

    pub(crate) async fn fetch_eva_map(&mut self) {
        let client = reqwest::Client::new();
        let url = format!("{}/station/*", API_URL);
        let request = client.get(url).headers(generate_headers(&self));
        let response = request.send().await.unwrap().text().await.unwrap();
        self.eva_map = EvaParser::parse_eva(&response);
    }

    pub(crate) fn get_eva_map(&self) -> &HashMap<String, String> {
        &self.eva_map
    }

    pub(crate) fn update_cache(&mut self, current_plan: BTreeMap<String, String>) {
        self.current_plan = current_plan;
    }

    pub(crate) fn update_eva_id(&mut self, eva_id: String) {
        self.eva_id = eva_id;
    }

    pub(crate) fn get_eva_id(&self) -> &String {
        &self.eva_id
    }

    pub(crate) fn get_current_plan(&self) -> &BTreeMap<String, String> {
        &self.current_plan
    }

    pub(crate) fn get_start(&self) -> String {
        self.from.clone()
    }

    pub(crate) fn get_destination(&self) -> String {
        self.to.clone()
    }
}
