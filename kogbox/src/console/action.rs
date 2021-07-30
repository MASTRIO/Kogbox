// Dependencies
use std::time::Duration;
use std::thread;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::string::String;
use colored::*;
mod script_compiler;

// Create Script
pub fn create_script(script_name: &str) {
    let mut script_path = String::from("scripts/");
    script_path.push_str(script_name);
    script_path.push_str(".kog");
    let _script = File::create(script_path);
    thread::sleep(Duration::from_secs(1));
    println!("{}{}{}", "[Script|Create]: Created script '".blue(), script_name.blue(), ".kog'".blue())
}

// Run Script
pub fn run_script(script_name: &str) -> io::Result<()> {
    println!("{}{}{}", "[Script|Run]: Running script '".blue(), script_name.blue(), ".kog'".blue());
    thread::sleep(Duration::from_millis(500));
    let mut script_path = String::from("scripts/");
    script_path.push_str(script_name);
    script_path.push_str(".kog");

    let script = File::open(script_path)?;
    let script_reader = BufReader::new(script);

    for line in script_reader.lines() {
        script_compiler::script_command(line?);
    }

    Ok(())
}