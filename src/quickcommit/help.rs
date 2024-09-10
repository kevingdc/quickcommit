pub fn print_help() {
    println!("quickcommit - A smart Git commit CLI tool");
    println!("Usage:");
    println!(
        "  quickcommit                 Auto-generate commit message and commit staged changes"
    );
    println!("  quickcommit commit          Same as above, explicitly specified");
    println!("  quickcommit help            Display this help message");
    println!("  quickcommit set-api-key     Set or update the API key for the LLM service");
}
