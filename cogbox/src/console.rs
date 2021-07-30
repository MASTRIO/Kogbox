// Dependencies
use std::time::Duration;
use std::thread;
use std::process;
use std::io;
mod action;

// Ask user what to do
pub fn ask_input() {
    println!("\n> What would you like to do?");
    // Get user input
    let mut action = String::new();
    io::stdin()
        .read_line(&mut action)
        .expect("Woopsies, I couldn't understand wtf you just typed or something :/");
    
    // Compile the action
    thread::sleep(Duration::from_millis(250));
    compile_action(action.trim());
}

// Action compiler
fn compile_action(action: &str) {
    let action = action.split(" ");
    let action: Vec<&str> = action.collect();
    match action[0] {
        "create" => action::create_script(action[1]),
        "run" => {
            let _result = action::run_script(action[1]);
        }
        "stop" | "exit" | "cancel" | "close" => {
            println!("[Process]: Exiting kogbox");
            thread::sleep(Duration::from_millis(500));
            process::exit(0)
        }
        _ => {
            println!("[Err]: Dumb Dumb that's not an action :/");
        }
    }
    thread::sleep(Duration::from_millis(500));
    ask_input()
}