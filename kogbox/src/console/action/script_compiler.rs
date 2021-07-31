// Dependencies
use std::time::Duration;
use std::thread;
use colored::*;

// Run Script Commands
pub fn script_command(instruction: String) {
    thread::sleep(Duration::from_millis(250));
    let instruction: &str = &instruction;
    let instruction = instruction.split("::");
    let mut instruction: Vec<&str> = instruction.collect();
    let args = instruction[1].split("[");
    let args: Vec<&str> = args.collect();
    instruction[1] = args[0];
    match instruction[0] {
        "test" => {
            match instruction[1] {
                "amogus" => println!("{}", "[Script|Test]: SUSSY!!!!".bright_blue()),
                "bruh" => println!("{}", "[Script|Test]: epic".bright_blue()),
                _ => invalid_argument(instruction[0], instruction[1])
            }
        }
        "action" => {
            match instruction[1] {
                "harvest" => {
                    println!("{}{} {}", "[Script|Task]: Harvested ".cyan(), args[2].cyan(), args[1].cyan())
                }
                _ => invalid_argument(instruction[0], instruction[1])
            }
        }
        _ => {
            println!("{}", "[Err]: Invalid instruction".black().on_red())
        }
    }
}

// Invalid Argument
fn invalid_argument(instruction: &str, argument: &str) {
    println!("{}{}{}{}{}", "[Err]: Invalid argument ".black().on_red(), argument.black().on_red(), " for instruction ".black().on_red(), instruction.black().on_red(), ", skipping".black().on_red())
}