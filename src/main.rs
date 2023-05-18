extern crate ctrlc;

use std::env;
use std::process;
use std::process::{Command, Stdio};
use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Welcome to github-readme-stats-docker!");

    register_force_stop();

    // Retrieve all environment variables as an iterator
    let env_vars = env::vars();

    // Print the name and value of each variable
    for (key, value) in env_vars {
        println!("{}: {}", key, value);
    }

    // Read the value of the system environment variable "GITHUB_TOKEN"
    let github_token =
        env::var("GITHUB_TOKEN").expect("Failed to read the TOKEN environment variable");

    start_child_process(github_token);
    start_http_proxy().await;
}

fn start_child_process(github_token: String) {
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

async fn start_http_proxy() {
    println!("Starting proxy server to prevent unauthorized access...");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([0, 0, 0, 0], 80)).await;
}

fn register_force_stop() {
    println!("Registering shutdown handler...");
    ctrlc::set_handler(move || {
        println!("Force exit...");
        process::exit(0);
    })
    .expect("Error setting up the signal handler.");
}
