pub mod models;
pub mod server;
#[macro_use]
extern crate juniper_codegen;
extern crate log;
extern crate pretty_env_logger;
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    println!("GraphQL server running on 4050...");
    server::start(([127, 0, 0, 1], 4050)).await;
}
