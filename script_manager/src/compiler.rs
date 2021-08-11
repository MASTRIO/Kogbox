// Dependencies
use std::time::Duration;
use std::thread;
use colored::*;

// Run Script Commands
pub fn compile_command(instruction: String) {
    thread::sleep(Duration::from_millis(250));
    let instruction: &str = &instruction;
    let instruction = instruction.split("::");
    let mut instruction: Vec<&str> = instruction.collect();
    //let _args_1 = argument_splitter(instruction.clone(), 1);
    let args = argument_splitter(instruction.clone(), 1);
    let instruction_splitter = instruction[1].split("[");
    let instruction_splitter: Vec<&str> = instruction_splitter.collect();
    instruction[1] = instruction_splitter[0];
    //instruction[1] = args[0];
    match instruction[0] {
        "test" => { // WARN: Doesn't work rn
            match instruction[1] {
                "amogus" => println!("{}", "[Script|Test]: SUSSY!!!!".bright_blue()),
                "bruh" => println!("{}", "[Script|Test]: epic".bright_blue()),
                _ => invalid_instruction()
            }
        }
        "world" => {
            match instruction[1] {
                "harvest" => {
                    let amount: u64 = match args[1].trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            invalid_instruction();
                            0
                        }
                    };
                    match args[0].trim() {
                        "tree" => {
                            println!("{}{} {}", "[Script|Task]: Harvesting ".cyan(), args[1].trim().cyan(), "trees".cyan());
                            thread::sleep(Duration::from_millis(50 * amount));
                            complete_task()
                        }
                        "rock" => {
                            println!("{}{} {}", "[Script|Task]: Harvesting ".cyan(), args[1].trim().cyan(), "rocks".cyan());
                            thread::sleep(Duration::from_millis(50 * amount));
                            complete_task()
                        }
                        _ => invalid_instruction()
                    }
                }
                _ => invalid_instruction()
            }
        }
        _ => {
            invalid_instruction()
        }
    }
}

// Invalid Argument
fn invalid_instruction() {
    println!("{}", "[Err]: Invalid instruction, Skipping".black().on_red())
}

// Argument Splitter
fn argument_splitter(instruction: Vec<&str>, id: usize) -> Vec<&str> {
    let args = instruction[id].split("[");
    let args: Vec<&str> = args.collect();
    let args = args[1].split("]");
    let args: Vec<&str> = args.collect();
    let args = args[0].split(",");
    let args: Vec<&str> = args.collect();   
    args
}

// Task Completed
fn complete_task() {
    println!("{}", "    >>> Task Completed".bright_green())
}