mod cache;

use std::str::FromStr;

use chrono::Local;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

use crate::{
    env::APIKeys,
    parser::ResponseParser,
    statics::{API_URL, TIMETABLES_LIMIT_MIN},
};

use self::cache::ServerCache;

#[derive(Default)]
pub(crate) struct DBebbleServer {
    client: Client,
    creds: APIKeys,
    cache: ServerCache,
}

impl DBebbleServer {
    pub fn reset(&mut self) -> () {
        self.cache.refresh_requests();
    }

    pub async fn get_current_plan(
        &mut self,
        eva_id: &str,
        to: &str,
    ) -> Result<Vec<String>, String> {
        let time = Local::now().to_string();
        let (date, time) = extract_date_time(time);
        let url = format!("{}/plan/{}/{}/{}", API_URL, eva_id, date, time);
        self.cache.update_state();
        if self.cache.get_state() == TIMETABLES_LIMIT_MIN {
            return Err("Too much requests, waiting for 60 secs before next request..".to_string());
        }
        let request = self.client.get(url).headers(self.generate_headers());
        let response: String = request.send().await.unwrap().text().await.unwrap();
        let train_times = ResponseParser::parse(response.as_str(), to.to_lowercase().as_str());
        Ok(train_times)
    }

    pub async fn get_station_eva(&mut self, station: &str) -> Result<String, String> {
        let url = format!("{}/{}", API_URL, self.generate_station_query(station));
        self.cache.update_state();
        if self.cache.get_state() == TIMETABLES_LIMIT_MIN {
            return Err("Too much requests, waiting for 60 secs..".to_string());
        }
        let request = self.client.get(url).headers(self.generate_headers());
        let response: String = request.send().await.unwrap().text().await.unwrap();
        let response = self.extract_eva(response);
        Ok(response)
    }

    fn extract_eva(&self, response: String) -> String {
        let mut id = String::new();
        for x in response.split(" ") {
            let pos_equal = x.find("=");
            if pos_equal.is_none() {
                continue;
            }
            let (key, value) = x.split_at(pos_equal.unwrap());
            if key == "eva" {
                id = value.chars().filter(|ch| ch.is_digit(10)).collect();
            }
        }
        id
    }

    fn generate_station_query(&self, query: &str) -> String {
        let parsed: Vec<&str> = query.split(" ").collect();
        let query: String = "/station/".to_string() + parsed.join("%20").to_lowercase().as_str();
        query
    }

    fn generate_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_str("DB-Client-Id").unwrap(),
            HeaderValue::from_str(self.creds.client_id.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_str("DB-Api-Key").unwrap(),
            HeaderValue::from_str(self.creds.api_key.as_str()).unwrap(),
        );

        headers
    }
}

fn extract_date_time(date_time: String) -> (String, String) {
    let (date, time) = date_time.split_at(date_time.find(" ").unwrap());
    let date: Vec<_> = date.split("-").collect();
    let date: String = date.join("");
    let time_no_secs: String = time.splitn(2, ":").collect();
    let (time, _) = time_no_secs.split_at(time_no_secs.find(":").unwrap());

    (date[2..].to_string(), time[1..3].to_string())
}
