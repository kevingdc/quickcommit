use crate::quickcommit::{api_key, commit, help};
use std::env;
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("help") => help::print_help(),
        Some("set-api-key") => api_key::set_api_key()?,
        Some("commit") | None => commit::auto_commit().await?,
        _ => {
            eprintln!("Error: Invalid command '{}'", args[1]);
            println!("\nHere's how to use quickcommit:");
            help::print_help();
            return Err("Invalid command".into());
        }
    }

    Ok(())
}
