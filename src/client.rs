use openssl::ssl::{SslConnector, SslMethod};
use postgres::Client;
use postgres_openssl::MakeTlsConnector;

fn argh() {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_ca_file("cert.pem")?;
    let connector = MakeTlsConnector::new(builder.build());

    let client = postgres::Client::connect(
        "host=localhost user=postgres sslmode=require",
        connector,
    )?;

    println!("Hello, World!");
}









use postgres::{Client, NoTls};

let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

client.batch_execute("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
")?;

let name = "Ferris";
let data = None::<&[u8]>;
client.execute(
    "INSERT INTO person (name, data) VALUES ($1, $2)",
    &[&name, &data],
)?;

for row in client.query("SELECT id, name, data FROM person", &[])? {
    let id: i32 = row.get(0);
    let name: &str = row.get(1);
    let data: Option<&[u8]> = row.get(2);

    println!("found person: {} {} {:?}", id, name, data);
}