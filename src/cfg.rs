pub struct Env {
    pub ip: String,
    pub port: u16,
}

pub async fn init() -> Result<Env, dotenv::Error>  {
    dotenv::dotenv()?;
    let ip = std::env::var("IP")
        .unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .expect("number");

    Ok(Env {
        ip,
        port,
    })
}
