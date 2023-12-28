use std::error::Error;
mod routes;

mod cfg {
    pub mod db;
    pub mod server;
    pub mod env;
}
use cfg::{db, server, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let config = env::init_env().await?;

    let db_url = db::get_db_url(
        &config.db_user,
        &config.db_password,
        &config.db_host,
        &config.db_port,
        &config.db_name,
    ).await;

    let db_pool = db::init_db(&db_url).await?;

    let listener = server::create_tcp_listener(&config.ip, config.port).await?;

    let router = routes::router();
    server::start_server(listener, router).await?;

    Ok(())
}
