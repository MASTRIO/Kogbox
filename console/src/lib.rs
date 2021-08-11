// Dependencies
use std::time::Duration;
use std::thread;
use std::io;
mod command_executor;

// Ask user what to do
pub fn get_command() {
    println!("\n> What would you like to do?");
    // Get user input
    let mut action = String::new();
    io::stdin()
        .read_line(&mut action)
        .expect("Woopsies, I couldn't understand wtf you just typed or something :/");
    
    // Compile the action
    thread::sleep(Duration::from_millis(250));
    command_executor::execute(action.trim());
    get_command()
}