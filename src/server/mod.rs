pub(crate) mod cache;
pub(crate) mod plan;
pub(crate) mod updates;

use std::str::FromStr;

use self::cache::ServerCache;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

fn extract_eva(response: String) -> String {
    response
        .split_whitespace()
        .filter_map(|x| {
            let pos_equal = x.find('=');
            pos_equal.and_then(|pos| {
                let (key, value) = x.split_at(pos);
                if key == "eva" {
                    Some(value.chars().filter(|ch| ch.is_digit(10)).collect())
                } else {
                    None
                }
            })
        })
        .next()
        .unwrap_or_else(String::new)
}
fn generate_station_query(query: &str) -> String {
    let parsed: Vec<&str> = query.split(" ").collect();
    let query: String = "/station/".to_string() + parsed.join("%20").to_lowercase().as_str();
    query
}

fn generate_headers(cache: &ServerCache) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_str("DB-Client-Id").unwrap(),
        HeaderValue::from_str(cache.creds.client_id.as_str()).unwrap(),
    );
    headers.insert(
        HeaderName::from_str("DB-Api-Key").unwrap(),
        HeaderValue::from_str(cache.creds.api_key.as_str()).unwrap(),
    );

    headers
}

fn extract_date_time(date_time: String) -> (String, String) {
    let (date, time) = date_time.split_at(date_time.find(" ").unwrap());
    let date: Vec<_> = date.split("-").collect();
    let date: String = date.join("");
    let time_no_secs: String = time.splitn(2, ":").collect();
    let (time, _) = time_no_secs.split_at(time_no_secs.find(":").unwrap());

    (date[2..].to_string(), time[1..3].to_string())
}
