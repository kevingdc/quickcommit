use std::error::Error;
use std::process::Command;

pub fn get_current_branch() -> Result<String, Box<dyn Error>> {
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

pub fn get_staged_files() -> Result<Vec<String>, Box<dyn Error>> {
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

pub fn get_diff_contents() -> Result<String, Box<dyn Error>> {
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

pub fn commit_changes(commit_message: &str) -> Result<(), Box<dyn Error>> {
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
