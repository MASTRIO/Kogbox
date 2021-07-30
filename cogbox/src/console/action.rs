// Dependencies
use std::time::Duration;
use std::thread;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::string::String;
mod script_compiler;

// Create Script
pub fn create_script(script_name: &str) {
    let mut script_path = String::from("scripts/");
    script_path.push_str(script_name);
    script_path.push_str(".cog");
    let _script = File::create(script_path);
    thread::sleep(Duration::from_secs(1));
    println!("[Pog]: Created script '{}.cog'", script_name)
}

// Run Script
pub fn run_script(script_name: &str) -> io::Result<()> {
    println!("> Executing script '{}.cog'", script_name);
    thread::sleep(Duration::from_millis(500));
    let mut script_path = String::from("scripts/");
    script_path.push_str(script_name);
    script_path.push_str(".cog");

    let script = File::open(script_path)?;
    let script_reader = BufReader::new(script);

    for line in script_reader.lines() {
        script_compiler::script_command(line?);
    }

    Ok(())
}
/*
pub fn run_script(script_name: &str) -> io::Result<()> {
    println!("Tried to run '{}.cog'", script_name);
    let mut script_path = String::from("scripts/");
    script_path.push_str(script_name);
    script_path.push_str(".cog");
    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(());
    //println!("Script:\n{}\n---", contents);
    //read_lines(contents)
}
*/

// Read Lines
/*
fn read_lines(contents: String) {
    let content = contents.clone();
    let lines_to_read: usize = contents.len();
    println!("{}", contents.len());
    let mut line_to_read: usize = 0;
    let contents = contents.split("\n");
    let contents: Vec<&str> = contents.collect();
    if line_to_read <= lines_to_read {
        println!("{}", contents[line_to_read]);
        line_to_read = line_to_read + 1;
        read_lines(content)
    }
}
*/