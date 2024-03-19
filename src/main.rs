use crate::server::DBebbleServer;
use std::{
    thread,
    time::{Duration, Instant},
};

mod env;
mod server;
mod statics;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut server = DBebbleServer::default();
    let query = "basel bad";
    let id = server.get_station_eva(query).await.unwrap();
    loop {
        if let Err(e) = server.get_station_eva("basel bad").await {
            println!("LOG: {e:?}");
            thread::sleep(Duration::from_secs(60));
            server.reset();
        }
    }

    Ok(())
}
