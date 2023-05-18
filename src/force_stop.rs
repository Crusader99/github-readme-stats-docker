extern crate ctrlc;

use std::process;

pub fn register_stop_handler() {
    println!("Registering shutdown handler...");
    ctrlc::set_handler(move || {
        println!("Force exit...");
        process::exit(0);
    })
    .expect("Error setting up the signal handler.");
}
