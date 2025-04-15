use inquire::MultiSelect;
use std::process::Command;

fn main() {
    if let Err(e) = run() {
        println!("");
        eprintln!("Error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let catch = |msg| move |e| format!("{msg}: {e}");

    // Get local branches
    let branches_str = exec(
        "git",
        &["for-each-ref", "--format=%(refname:short)", "refs/heads/"],
    )
    .map_err(catch("Failed to get git branches"))?;
    let branches = branches_str.trim().split("\n").collect::<Vec<_>>();

    // Show the prompt
    let mut answer = MultiSelect::new("Select the branches to delete:", branches)
        .with_formatter(&|a| a.iter().map(|a| *a.value).collect::<Vec<_>>().join(", "))
        .prompt()?;

    // Exclude the current branch
    let current_branch =
        exec("git", &["branch", "--show-current"]).map_err(catch("Failed to get git branch"))?;
    if let Some(i) = answer.iter().position(|i| **i == current_branch) {
        println!(">> '{current_branch}' is the current branch. Skipping");
        answer.remove(i);
    }

    if answer.is_empty() {
        println!(">> No branches were selected. Exit");
        return Ok(());
    }

    // Delete branches
    let mut args = answer.clone();
    args.insert(0, "br");
    args.insert(1, "-D");
    exec("git", &args).map_err(catch("Failed to delete branches"))?;

    println!(">> Removed branches: {}", answer.join(", "));
    Ok(())
}

fn exec(cmd: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(cmd).args(args).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
