use server::DBebbleServer;

mod env;
mod parser;
mod server;
mod statics;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut server_plan = DBebbleServer::default();
    let mut server_updates = server_plan.clone();
    let plan_handle = tokio::spawn(async move { server_plan.fetch_plan_task().await });
    let updates_handle = tokio::spawn(async move { server_updates.fetch_updates_task().await });

    let _ = tokio::try_join!(plan_handle, updates_handle);
}
