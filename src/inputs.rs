use std::io::Write;

use rpassword;

use crate::errors::{ErrorType, SupergenError};

// Takes two password inputs and returns Ok if the password matches. Error if not.
fn check_matches<'a>(entry1: String, entry2: String)
                            -> Result<String, Box< dyn std::error::Error>> {
    
    match entry1.eq(&entry2) {
        true => Ok(entry1),
        false => Err(Box::new(
            SupergenError {
                error_type: ErrorType::MismatchError
            }
        ))
    }
}

// Takes a reference to the entered username and returns Ok(()) if it is equal
// to or below the 320 character limit.
fn validate_username_length(username: &str) 
                            -> Result<(), Box<dyn std::error::Error>> {
    match username.len() <= 320 {
        true => Ok(()),
        false => Err(Box::new(
            SupergenError {
                error_type: ErrorType::InvalidUsernameError
            }
        ))
    }
}

// Requests 
pub fn username() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        // Username prompt. Flushes the prompt on stdout so that entry occurs
        // on the same line. Purely for aesthetic purposes.
        let mut stdout = std::io::stdout();
        print!("Username: ");
        stdout.flush().unwrap();

        // Create an empty String. Update it with the terminal input.
        let mut username = String::new();
        std::io::stdin().read_line(&mut username)
            .expect("Failed to read input from terminal.");
        
        // Check the entered username is less than or equal to the 320
        // character limit. Return the username if ok. Loop in case of Err.
        match crate::inputs::validate_username_length(&username) {
            Ok(_) => break Ok(username),
            Err(_) => continue
        }
    }
}

pub fn password() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        // Prompt for the password twice.
        let entry1 = rpassword::prompt_password("Password: ").unwrap();
        let entry2 = rpassword::prompt_password("Re-enter Password: ").unwrap();

        // Call function to check entered passwords match.
        let validated_password = check_matches(entry1, entry2);
        
        // If previous function returns ok, 
        match validated_password {
            Ok(value) => break Ok(value),
            Err(_) => continue
        }
    }
}