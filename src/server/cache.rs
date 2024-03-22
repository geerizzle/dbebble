use crate::env::APIKeys;

#[derive(Default)]
pub(crate) struct ServerCache {
    current_plan: Vec<String>,
    pub(crate) creds: APIKeys,
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

    pub(crate) fn update_cache(&mut self, current_plan: Vec<String>) {
        self.current_plan = current_plan;
    }

    pub(crate) fn get_start(&self) -> String {
        self.from.clone()
    }

    pub(crate) fn get_destination(&self) -> String {
        self.to.clone()
    }
}
