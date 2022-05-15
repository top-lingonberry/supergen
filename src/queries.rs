use postgres::Client;

static CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, username VARCHAR(320) UNIQUE NOT NULL, salt BYTEA NOT NULL, password BYTEA NOT NULL);";
static CREATE_USER: &str = "INSERT INTO users VALUES (default, $1, $2, $3)";

pub fn create_table(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let _result: u64 = client.execute(CREATE_TABLE, &[])?;
    println!("Succesfully created table.");
    Ok(())
}

pub fn create_user(
        client: &mut Client, username: String, 
        salt: &[u8], password: &[u8]
    ) -> Result<(), Box<dyn std::error::Error>> {

    let _query = client.execute(CREATE_USER, &[&username, &salt, &password])?;
    println!("Succesfully created user.");
    Ok(())
}