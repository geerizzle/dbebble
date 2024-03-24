use std::collections::BTreeMap;

use crate::env::APIKeys;

#[derive(Default)]
pub(crate) struct ServerCache {
    current_plan: BTreeMap<String, String>,
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
