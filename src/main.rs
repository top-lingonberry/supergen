use clap::Command;

mod arguments;
mod hashing;

const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = Command::new("supergen")
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(crate::arguments::Args::username())
        .arg(crate::arguments::Args::password())
        .get_matches();
    
    let username = app.value_of("username").expect("Missing parameter --username.");
    println!("The provided username is: {}", username);
}