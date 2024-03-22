use server::cache::ServerCache;

use crate::server::{plan::PlanFetcher, updates::UpdatesFetcher};

mod env;
mod parser;
mod server;
mod statics;
mod ui;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    println!("Input a starting station");
    let _ = stdin.read_line(&mut buffer)?;
    let from = buffer.trim().to_string();
    buffer.clear();
    println!("Input a end goal");
    let _ = stdin.read_line(&mut buffer)?;
    let to = buffer.trim().to_string();

    let cache = ServerCache::new(from, to);
    let mut plan_fetcher = PlanFetcher::new(cache.clone());
    let mut updates_fetcher = UpdatesFetcher::new(cache.clone());
    let plan_handle = tokio::spawn(async move { plan_fetcher.start().await });
    let updates_handle = tokio::spawn(async move { updates_fetcher.start().await });

    let _ = tokio::try_join!(plan_handle, updates_handle);

    Ok(())
}
