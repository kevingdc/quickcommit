use keyring::Entry;
use std::error::Error;
use std::io::{self, Write};

const SERVICE_NAME: &str = "quickcommit";
const USERNAME: &str = "api-key";

pub fn set_api_key() -> Result<(), Box<dyn Error>> {
    print!("Enter your API key: ");
    io::stdout().flush()?;

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim();

    let entry = Entry::new(SERVICE_NAME, USERNAME);
    entry.set_password(api_key)?;

    println!("API key has been securely stored.");
    Ok(())
}

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
    let entry = Entry::new(SERVICE_NAME, USERNAME);
    match entry.get_password() {
        Ok(password) => Ok(password),
        Err(_) => Err("API key not found. Please set it using 'quickcommit set-api-key'".into()),
    }
}
