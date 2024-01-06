extern crate git2;

use std::env;

use std::process::Command;

fn clone_repo(user: &str, repo: &str, branch: &str) {
    let target_directory = if branch.is_empty() {
        format!("~/repos/{}/{}", user, repo)
    } else {
        format!("~/repos/{}/{}_{}", user, repo, branch)
    };

    let clone_url = format!("https://github.com/{}/{}", user, repo);

    let target = &format!("{}", target_directory);
    let clone_command = if branch.is_empty() {
        vec!["git", "clone", &clone_url, target]
    } else {
        vec!["git", "clone", "-b", &branch, &clone_url, target]
    };

    let status = Command::new("sh")
        .arg("-c")
        .arg(clone_command.join(" "))
        .status();

    match status {
        Ok(_) => println!("Repository cloned successfully."),
        Err(e) => println!("Failed to clone repository: {}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <GitHub URL>");
        return;
    }

    let url = &args[1];
    let parts: Vec<&str> = url.split('/').collect();

    let user = parts.get(3).unwrap_or(&"").to_string();
    let repo = parts.get(4).unwrap_or(&"").to_string();
    let branch = parts.get(6).unwrap_or(&"").to_string();

    println!("User: {}, Repo: {}, Branch: {}", user, repo, branch);

    clone_repo(&user, &repo, &branch);
}
