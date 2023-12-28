use tokio::net::TcpListener;
use axum::Router;
use std::error;

pub async fn create_tcp_listener(ip: &str, port: u16) -> Result<TcpListener, Box<dyn error::Error>> {
    let addr = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::debug!("TCP LISTENER STARTED ON: {}", listener.local_addr().unwrap());
    Ok(listener)
}

pub async fn start_server(listener: TcpListener, router: Router) -> Result<(), Box<dyn error::Error>> {
    axum::serve(
        listener,
        router
    ).await?;
    Ok(())
}
