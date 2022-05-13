use rpassword;
use std::io::Write;

mod client;
mod errors;
mod hashing;
mod queries;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Username prompt. Flushes the prompt on stdout so that entry occurs
    // on the same line. Purely for aesthetic purposes.
    let mut stdout = std::io::stdout();
    print!("Username: ");
    stdout.flush().unwrap();
    
    // Username entry to String.
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)
        .expect("Failed to read input from terminal.");

    // Password entry prompt, hidden values using rpassword.
    let password1 = rpassword::prompt_password("Password: ").unwrap();
    let password2 = rpassword::prompt_password("Re-enter password: ").unwrap();

    // Ensure the two password entries match. Raises an error if not.
    let validated_password: &str = crate::hashing::check_matches(password1, &password2)?;

    // Return the randomly generated salt and hashed password as byte arrays. 
    let (salt, password_hash) = crate::hashing::encrypt_password(validated_password)?;
    let mut client = crate::client::client()?;
    crate::queries::create_table(&mut client)?;
    crate::queries::create_user(&mut client, username, &salt, &password_hash)?;

    println!("The salt is: {:?}", salt);
    println!("The password hash is: {:?}", password_hash);

    Ok(())
}