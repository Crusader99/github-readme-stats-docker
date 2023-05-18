use std::process::{Command, Stdio};

pub fn start_child_process(github_token: String) {
    println!("Starting github-readme-stats express server...");

    // Set the working directory for the subprocess
    let working_dir = ".";

    // Start the subprocess asynchronously with the environment variable
    Command::new("node")
        .current_dir(working_dir)
        .arg("express.js")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PAT_1", github_token)
        .spawn()
        .expect("Failed to start the subprocess");
}