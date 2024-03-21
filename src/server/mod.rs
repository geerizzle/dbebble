use std::str::FromStr;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

use crate::{
    env::APIKeys,
    parser::ResponseParser,
    statics::{API_URL, TIMETABLES_LIMIT_MIN},
};

#[derive(Debug, Default)]
pub(crate) struct DBebbleServer {
    client: Client,
    creds: APIKeys,
    num_sent: u8,
}

impl DBebbleServer {
    pub fn reset(&mut self) -> () {
        self.num_sent = 0;
    }

    pub async fn get_current_plan(
        &mut self,
        eva_id: &str,
        date: &str,
        time: &str,
    ) -> Result<String, String> {
        let url = format!("{}/plan/{}/{}/{}", API_URL, eva_id, date, time);
        let request = self.client.get(url).headers(self.generate_headers());
        if self.num_sent == TIMETABLES_LIMIT_MIN {
            return Err("Too much requests, waiting for 60 secs..".to_string());
        }
        let response: String = request.send().await.unwrap().text().await.unwrap();
        self.num_sent += 1;
        Ok(response)
    }

    pub async fn get_station_eva(&mut self, station: &str) -> Result<String, String> {
        let url = format!("{}/{}", API_URL, self.generate_station_query(station));
        let request = self.client.get(url).headers(self.generate_headers());
        if self.num_sent == TIMETABLES_LIMIT_MIN {
            return Err("Too much requests, waiting for 60 secs..".to_string());
        }
        let response: String = request.send().await.unwrap().text().await.unwrap();
        let response = self.extract_eva(response);
        self.num_sent += 1;
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
        let query: String = "/station/".to_string() + parsed.join("%20").as_str();
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
