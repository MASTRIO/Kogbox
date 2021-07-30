// Dependencies
use std::fs;
mod scripts_manager;
mod console;

// Main
fn main() {
    initilizer();
    println!("Welcome to the Cogbox CLI!");
    let _scripts_dir = scripts_manager::create_dir();
    console::ask_input();
}

fn initilizer() {
    println!("[Init]: Attempting to create '/scripts' directory");
    let _scripts_dir = scripts_manager::create_dir();
    println!("[Init]: Loading CLI...")
}

// TEMP: Read the file
fn _read_file(filename: &str) {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Uh oh, le program couldn't read the file, time to explode your computer!");

    println!("With text:\n{}", contents);
}