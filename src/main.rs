use crate::server::DBebbleServer;
use std::{thread, time::Duration};

mod env;
mod parser;
mod server;
mod statics;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut server = DBebbleServer::default();
    let from = "basel bad";
    let from_id = server.get_station_eva(from).await.unwrap();
    let to = "Lauchringen";
    loop {
        if from == "quit" {
            break;
        }
        match server.get_current_plan(from_id.as_str(), to).await {
            Ok(times) => {
                println!("Next train to {to:?}: {:?}", times.iter().next().unwrap());
            }
            Err(e) => {
                println!("LOG: {e:?}");
                server.reset();
            }
        }
    }

    Ok(())
}
