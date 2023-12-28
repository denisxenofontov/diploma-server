pub struct Env {
    pub ip: String,
    pub port: u16,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_name: String
}

pub async fn init_env() -> Result<Env, dotenv::Error>  {
    dotenv::dotenv()?;
    let ip = std::env::var("IP")
        .unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .expect("number");

    let db_port = std::env::var("DB_PORT")
        .unwrap_or_else(|_| String::from("5432"));
    let db_user = std::env::var("DB_USER")
        .unwrap_or_else(|_| String::from("postgres"));
    let db_password = std::env::var("DB_PASS")
        .unwrap_or_else(|_| String::from("postgres"));
    let db_host = std::env::var("DB_HOST")
        .unwrap_or_else(|_| String::from("localhost"));
    let db_name = std::env::var("DB_NAME")
        .unwrap_or_else(|_| String::from("diploma"));

    Ok(Env {
        ip,
        port,
        db_port,
        db_user,
        db_password,
        db_host,
        db_name
    })
}
