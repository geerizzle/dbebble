use std::str::FromStr;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

use crate::{env::APIKeys, statics::API_URL};

#[derive(Debug, Default)]
pub(crate) struct DBebbleServer {
    client: Client,
    creds: APIKeys,
    num_sent: u8,
}

impl DBebbleServer {
    fn reset(&mut self) -> () {
        self.num_sent = 0;
    }

    pub async fn get_station_eva(&mut self, station: &str) -> reqwest::Result<()> {
        let url = API_URL.to_string() + self.parse_query(station).as_str();
        println!("Url: {url:?}");
        let request = self.client.get(url).headers(self.generate_headers());
        let response = request.send().await?.text().await?;
        self.num_sent += 1;
        println!("Response: {response:?}");
        Ok(())
    }

    fn parse_query(&self, query: &str) -> String {
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
