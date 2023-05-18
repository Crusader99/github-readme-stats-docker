use std::env;

pub mod force_stop;
pub mod proxy;
pub mod sub_process;

#[tokio::main]
async fn main() {
    println!("Welcome to github-readme-stats-docker!");

    force_stop::register_stop_handler();

    // Retrieve all environment variables as an iterator
    let env_vars = env::vars();

    // Print the name and value of each variable
    for (key, value) in env_vars {
        println!("{}: {}", key, value);
    }

    // Read the value of the system environment variable "GITHUB_TOKEN"
    let github_token =
        env::var("GITHUB_TOKEN").expect("Failed to read the TOKEN environment variable");

    sub_process::start_child_process(github_token);
    proxy::start_http_proxy().await;
}
