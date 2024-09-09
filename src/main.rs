use keyring::Entry;
use reqwest;
use serde_json::json;
use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

const SERVICE_NAME: &str = "QuickCommit";
const USERNAME: &str = "api-key";
const BASE_PROMPT: &str = "
    Take a deep breath and work on this problem step-by-step.
    Summarize the provided diff into a clear and concise written commit message.
    Use the imperative style for the subject, use the imperative style for the body, and limit the combination of the entire subject line to 50 characters or less.
    Optionally, use a scope, and limit the scope types to 50 characters or less. Be as descriptive as possible, but keep it to a single line.
    It should be ready to be pasted into commit edits without further editing.
    Do not add the ``` to the start and end of the commit message.
    It is important that you start the subject with a commit type based on the changes made.

    The following are the commit types that you can use:
        feat     (new feature)
        fix      (bug fix)
        refactor (refactoring production code)
        style    (formatting, missing semi colons, etc; no code change)
        docs     (changes to documentation)
        test     (adding or refactoring tests; no production code change)
        perf     (code change that improves performance)
        revert   (revert a commit)
        build    (changes that affect the build system)
        ci       (changes to the CI configuration files or scripts)
        chore    (updating grunt tasks etc; no production code change)

    It is crucial that you follow the rules above.

    Use the following information to generate the commit message:
";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("help") => print_help(),
        Some("set-api-key") => set_api_key()?,
        Some("commit") | None => auto_commit().await?,
        _ => {
            eprintln!("Error: Invalid command '{}'", args[1]);
            println!("\nHere's how to use QuickCommit:");
            print_help();
            return Err("Invalid command".into());
        }
    }

    Ok(())
}

fn print_help() {
    println!("QuickCommit - A smart Git commit CLI tool");
    println!("Usage:");
    println!(
        "  quickcommit                 Auto-generate commit message and commit staged changes"
    );
    println!("  quickcommit commit          Same as above, explicitly specified");
    println!("  quickcommit help            Display this help message");
    println!("  quickcommit set-api-key     Set or update the API key for the LLM service");
}

fn set_api_key() -> Result<(), Box<dyn Error>> {
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

fn get_api_key() -> Result<String, Box<dyn Error>> {
    let entry = Entry::new(SERVICE_NAME, USERNAME);
    match entry.get_password() {
        Ok(password) => Ok(password),
        Err(_) => Err("API key not found. Please set it using 'quickcommit set-api-key'".into()),
    }
}

async fn auto_commit() -> Result<(), Box<dyn Error>> {
    // Get the current branch name
    let branch_name = get_current_branch()?;

    // Get the list of staged files
    let staged_files = get_staged_files()?;

    if staged_files.is_empty() {
        println!("No changes staged for commit. Please stage your changes first.");
        return Ok(());
    }

    // Generate the commit message using LLM
    let commit_message = generate_commit_message_llm(&branch_name, &staged_files).await?;

    // Commit the changes
    commit_changes(&commit_message)?;

    println!("Committed changes with message:\n{}", commit_message);

    Ok(())
}

fn get_current_branch() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if output.status.success() {
        let branch = String::from_utf8(output.stdout)?.trim().to_string();
        if !branch.is_empty() {
            return Ok(branch);
        }
    }

    // Check if this is a new repository with no commits
    let rev_list_output = Command::new("git")
        .args(&["rev-list", "-n", "1", "--all"])
        .output()?;

    if rev_list_output.stdout.is_empty() {
        // This is a new repository with no commits
        Ok("main".to_string())
    } else {
        Err("Failed to get current branch".into())
    }
}

fn get_staged_files() -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["diff", "--cached", "--name-only"])
        .output()?;

    if !output.status.success() {
        return Err("Failed to get staged files".into());
    }

    Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect())
}

async fn generate_commit_message_llm(
    branch_name: &str,
    staged_files: &[String],
) -> Result<String, Box<dyn Error>> {
    let api_key = get_api_key()?;
    let diff_content = get_file_contents()?;

    let prompt = format!(
        "{}:\n\
        Branch name: {}\n\
        Staged files: {}\n\
        Git diff:\n{}",
        BASE_PROMPT,
        branch_name,
        staged_files.join(", "),
        diff_content
    );

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-4o",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 100
        }))
        .send()
        .await?;

    let response_body: serde_json::Value = response.json().await?;
    let commit_message = response_body["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to extract commit message from API response")?
        .trim()
        .to_string();

    Ok(commit_message)
}

fn get_file_contents() -> Result<String, Box<dyn Error>> {
    // Get the diff for staged changes
    let output = Command::new("git").args(&["diff", "--cached"]).output()?;

    if !output.status.success() {
        return Err("Failed to get git diff for staged changes".into());
    }

    let diff_content = String::from_utf8(output.stdout)?;

    // If the diff is too large, we'll truncate it
    const MAX_DIFF_LENGTH: usize = 4000; // Adjust this value as needed
    let truncated_diff = if diff_content.len() > MAX_DIFF_LENGTH {
        format!("{}... (truncated)", &diff_content[..MAX_DIFF_LENGTH])
    } else {
        diff_content
    };

    Ok(format!("Git diff for staged changes:\n{}", truncated_diff))
}

fn commit_changes(commit_message: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["commit", "-m", commit_message])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to commit changes: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
