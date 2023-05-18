pub mod force_stop;
pub mod proxy;
pub mod sub_process;

use force_stop::register_stop_handler;
use proxy::start_http_proxy;
use std::env::var;
use sub_process::start_child_process;

#[tokio::main]
async fn main() {
    println!("Welcome to github-readme-stats-docker!");
    println!("https://github.com/Crusader99/github-readme-stats-docker");
    println!("---");
    register_stop_handler();

    // Read the values of the system environment variables
    var("GITHUB_USER").expect("Failed to read the GITHUB_USER environment variable");
    let github_token =
        var("GITHUB_TOKEN").expect("Failed to read the GITHUB_TOKEN environment variable");

    // Forward requests to actual github-readme-stats subprocess.
    start_child_process(github_token);
    start_http_proxy().await;
}
