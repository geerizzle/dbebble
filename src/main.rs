use env::APIKeys;

use crate::{server::DBebbleServer, statics::API_URL};

mod env;
mod server;
mod statics;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut server = DBebbleServer::default();
    let _ = server.get_station_eva("basel bad").await?;
    println!("Server state: {server:?}");

    Ok(())
}
