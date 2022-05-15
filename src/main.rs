mod client;
mod errors;
mod hashing;
mod inputs;
mod queries;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = crate::inputs::username()?;
    let password = crate::inputs::password()?;

    // Return the randomly generated salt and hashed password as byte arrays. 
    let (salt, password_hash) = crate::hashing::encrypt_password(password)?;

    // Fetch the client and execute queries to create the table and add the new user.
    // Prints messages to confirm successful database transactions.
    let mut client = crate::client::client()?;
    crate::queries::create_table(&mut client)?;
    crate::queries::create_user(&mut client, username, &salt, &password_hash)?;

    // Return nothing. Simply for ease of handling errors.
    Ok(())
}