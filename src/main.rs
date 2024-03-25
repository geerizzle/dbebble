use std::sync::{Arc, Mutex};

use server::cache::ServerCache;

use crate::server::{plan::PlanFetcher, updates::UpdatesFetcher};

mod env;
mod parser;
mod search;
mod server;
mod statics;
mod ui;

#[tokio::main()]
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

    let mut server_cache = ServerCache::new(from, to);
    let _ = server_cache.fetch_eva_map().await;
    let mut qgram_index = search::QGramIndex::default();
    let _ = qgram_index.build(&server_cache.get_eva_map());

    let cache = Arc::new(Mutex::new(server_cache));
    let cache_arc = Arc::clone(&cache);
    let mut plan_fetcher = PlanFetcher::new(cache);
    let mut updates_fetcher = UpdatesFetcher::new(cache_arc);
    let plan_handle = tokio::spawn(async move { plan_fetcher.start().await });
    let updates_handle = tokio::spawn(async move { updates_fetcher.start().await });

    let _ = tokio::try_join!(plan_handle, updates_handle);

    Ok(())
}
