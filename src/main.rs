use std::process::Command;
use regex::Regex;

fn main() {
    // Get the current branch
    let cur = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("failed to execute command");
    let current = String::from_utf8_lossy(&cur.stdout);

    // Get the default branch
    let default = Command::new("git")
        .arg("remote")
        .arg("show")
        .arg("origin")
        .output()
        .expect("failed to execute command");

    let out = String::from_utf8_lossy(&default.stdout);

    let re = Regex::new(r"HEAD branch: (.*)").unwrap();
    let cap = re.captures(&out).unwrap();
    let default_branch = &cap[1];

    // Checkout to the default branch
    Command::new("git")
        .arg("checkout")
        .arg(default_branch)
        .output()
        .expect("failed to execute command");

    // Pull the latest changes
    Command::new("git")
        .arg("pull")
        .output()
        .expect("failed to execute command");

    // Back to the current branch
    Command::new("git")
        .arg("checkout")
        .arg(current.to_string())
        .output()
        .expect("failed to execute command");



}