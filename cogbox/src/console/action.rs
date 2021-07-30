// Dependencies
use std::fs::File;
use std::string::String;

// Create Script
pub fn create_script(script_name: &str) {
    let mut script_path = String::from("scripts/");
    script_path.push_str(script_name);
    script_path.push_str(".cog");
    let _script = File::create(script_path);
    println!("[Pog]: Created script '{}.cog'", script_name)
}

// Run Script
pub fn run_script(script_name: &str) {

}