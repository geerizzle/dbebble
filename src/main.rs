use crate::server::DBebbleServer;
use chrono::prelude::Local;
use std::{thread, time::Duration};

mod env;
mod parser;
mod server;
mod statics;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut server = DBebbleServer::default();
    let from = "basel bad";
    let time = Local::now().to_string();
    let (date, time) = extract_date_time(time);
    let (date, time) = (date.as_str(), time.as_str());
    let from_id = server.get_station_eva(from).await.unwrap();
    loop {
        if from == "quit" {
            break;
        }
        match server.get_current_plan(from_id.as_str(), date, time).await {
            Ok(_) => (),
            Err(e) => {
                println!("LOG: {e:?}");
                thread::sleep(Duration::from_secs(60));
                server.reset();
            }
        }
    }

    Ok(())
}

fn extract_date_time(date_time: String) -> (String, String) {
    let (date, time) = date_time.split_at(date_time.find(" ").unwrap());
    let date: Vec<_> = date.split("-").collect();
    let date: String = date.join("");
    let time: String = time.splitn(2, ":").collect();
    let (time, _) = time.split_at(time.find(":").unwrap());

    (date[2..].to_string(), time[1..3].to_string())
}
