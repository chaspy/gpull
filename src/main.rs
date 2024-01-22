use std::process::Command;

fn main() {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute command");

    let output = String::from_utf8_lossy(&output.stdout);

    println!("{}", output)
}