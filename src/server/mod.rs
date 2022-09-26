use std::net::SocketAddr;
pub mod routes;

// Starts the server
pub async fn start(addr: impl Into<SocketAddr>) {
    warp::serve(routes::make_routes()).run(addr).await;
}
