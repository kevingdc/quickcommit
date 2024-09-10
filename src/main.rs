use quickcommit::quickcommit::cli;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    cli::run().await
}
