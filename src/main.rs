use crate::server::DBebbleServer;
use std::{thread, time::Duration};

mod env;
mod server;
mod statics;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut server = DBebbleServer::default();
    let from = "basel bad";
    let time = "08";
    let date = "240319";
    let from_id = server.get_station_eva(from).await.unwrap();
    println!("From ID: {from_id:?}");
    loop {
        if from == "quit" {
            break;
        }
        match server.get_current_plan(from_id.as_str(), date, time).await {
            Ok(res) => println!("{res:?}"),
            Err(e) => {
                println!("LOG: {e:?}");
                thread::sleep(Duration::from_secs(60));
                server.reset();
            }
        }
    }

    Ok(())
}
