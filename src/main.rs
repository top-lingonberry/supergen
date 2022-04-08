use clap::Command;
use ring::error::Unspecified;

mod arguments;
mod hashing;

const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Unspecified> {
    let app = Command::new("supergen")
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(crate::arguments::Args::username())
        .arg(crate::arguments::Args::password())
        .get_matches();
    
    let username = app.value_of("username").expect("Missing parameter --username.");
    let password: &str = app.value_of("password").expect("Missing parameter --password.");
    let (salt, password_hash) = crate::hashing::hash_pasword(password)?;
    println!("The provided username is: {}", username);
    println!("The salt is: {}", salt);
    println!("The password hash is: {}", password_hash);

    Ok(())
}