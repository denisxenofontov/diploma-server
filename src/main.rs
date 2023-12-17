use std::{net::{SocketAddr, IpAddr}, error::Error};
use axum::Router;
use tokio::net::{TcpListener};
mod cfg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let config = cfg::init().await?;

    let listener = create_tcp_listener(&config.ip, config.port).await?;

    start_server(listener).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, Worldfs!"
}

async fn create_tcp_listener(ip: &str, port: u16) -> Result<TcpListener, Box<dyn Error>> {
    let addr = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(addr).await?;
    tracing::debug!("SERVER STARTED ON {}", listener.local_addr().unwrap());
    Ok(listener)
}

async fn start_server(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    axum::serve(
        listener,
        Router::new().route("/", axum::routing::get(root))
    ).await.map_err(|err| err.into())
}
