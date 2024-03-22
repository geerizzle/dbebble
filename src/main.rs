use server::DBebbleServer;

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

    let mut server_plan = DBebbleServer::new(from, to);
    let mut server_updates = server_plan.clone();
    let plan_handle = tokio::spawn(async move { server_plan.fetch_plan_task().await });
    let updates_handle = tokio::spawn(async move { server_updates.fetch_updates_task().await });
    let _ = tokio::try_join!(plan_handle, updates_handle);
    Ok(())
}
