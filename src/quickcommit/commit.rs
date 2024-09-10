use crate::quickcommit::{api_key, git};
use reqwest;
use serde_json::json;
use std::error::Error;

const BASE_PROMPT: &str = "
    Take a deep breath and work on this problem step-by-step.
    Summarize the provided diff into a clear and concise written commit message.
    Use the imperative style for the subject, use the imperative style for the body, and limit the entire subject line to 50 characters or less.
    It should be ready to be pasted into a commit without further editing.
    Do not add the ``` to the start and end of the commit message.
    It is important that you start the subject with a commit type based on the changes made.
    The title in the subject after the commit type should start with a capital letter.

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

pub async fn auto_commit() -> Result<(), Box<dyn Error>> {
    // Get the current branch name
    let branch_name = git::get_current_branch()?;

    // Get the list of staged files
    let staged_files = git::get_staged_files()?;

    if staged_files.is_empty() {
        println!("No changes staged for commit. Please stage your changes first.");
        return Ok(());
    }

    // Generate the commit message using LLM
    let commit_message = generate_commit_message_llm(&branch_name, &staged_files).await?;

    // Commit the changes
    git::commit_changes(&commit_message)?;

    println!("Committed changes with message:\n{}", commit_message);

    Ok(())
}

async fn generate_commit_message_llm(
    branch_name: &str,
    staged_files: &[String],
) -> Result<String, Box<dyn Error>> {
    let api_key = api_key::get_api_key()?;
    let diff_content = git::get_diff_contents()?;

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
