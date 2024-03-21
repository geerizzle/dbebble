#[derive(Default)]
pub(crate) struct ServerCache {
    current_plan: Vec<String>,
    sent_requests: u8,
}

impl ServerCache {
    pub(crate) fn update_cache(&mut self, current_plan: Vec<String>) -> () {
        self.current_plan = current_plan;
    }

    pub(crate) fn refresh_requests(&mut self) -> () {
        self.sent_requests = 0;
    }

    pub(crate) fn update_state(&mut self) -> () {
        self.sent_requests += 1;
    }

    pub(crate) fn get_state(&self) -> u8 {
        self.sent_requests
    }
}
