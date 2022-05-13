use std::env;
use openssl::ssl::{SslConnector, SslMethod};
use postgres::Client;
use postgres::config::{Config, SslMode};
use postgres_openssl::MakeTlsConnector;

fn tls_connector() -> Result<MakeTlsConnector, Box<dyn std::error::Error>> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_ca_file("cert.pem")?;
    let connector = MakeTlsConnector::new(builder.build());

    Ok(connector)
}

pub fn client() -> Result<Client, Box< dyn std::error::Error>> {
    let client = Config::new()
        .user(env::var("SUPERGEN_DB_USER")?.as_str())
        .password(env::var("SUPERGEN_DB_PASSWORD")?.as_str())
        .host(env::var("SUPERGEN_DB_HOST")?.as_str())
        .dbname(env::var("SUPERGEN_DB_NAME")?.as_str())
        .ssl_mode(SslMode::Require)
        .connect(tls_connector()?)?;
    
    Ok(client)
}