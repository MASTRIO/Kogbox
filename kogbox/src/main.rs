// Dependencies
use std::time::Duration;
use std::thread;
use colored::*;
mod scripts_manager;
mod console;

// Main
fn main() {
    thread::sleep(Duration::from_millis(500));
    initilizer();
    thread::sleep(Duration::from_secs(1));
    println!("Welcome to Kogbox!");
    let _scripts_dir = scripts_manager::create_dir();
    console::ask_input();
}

fn initilizer() {
    println!("{}", "[Process]: Starting up Kogbox".bright_red());
    thread::sleep(Duration::from_millis(100));
    println!("{}", "[Init]: Attempting to create '/scripts' directory".green());
    let _scripts_dir = scripts_manager::create_dir();
    thread::sleep(Duration::from_millis(250));
    println!("{}", "[Init]: Loading CLI...".green());
}