// Dependencies
use std::time::Duration;
use std::thread;
use std::process;
use colored::*;
use script_manager;

// Action compiler
pub fn execute(action: &str) {
    let action = action.split(" ");
    let action: Vec<&str> = action.collect();
    match action[0] {
        "create" => script_manager::create_script(action[1]),
        "run" => {
            let _result = script_manager::run_script(action[1]);
        }
        "stop" | "exit" | "cancel" | "close" => {
            println!("{}", "[Process]: Exiting kogbox".bright_red());
            thread::sleep(Duration::from_millis(500));
            process::exit(0)
        }
        "help" | "um" | "amogus" => {
            println!("{}", "/~~~~~~= Help =~~~~~~\\".magenta());
            println!("{}", "- help/um/amogus (Shows the help menu)".magenta());
            println!("{}", "- stop/exit/cancel/close (Closes Kogbox)".magenta());
            println!("{}", "- create <script name> (Creates a new script)".magenta());
            println!("{}", "- run <script name> (Runs a script on the currently loaded world)".magenta())
        }
        _ => {
            println!("{}", "[Err]: Dumb Dumb that's not an action :/".black().on_red());
        }
    }
    thread::sleep(Duration::from_millis(500))
}