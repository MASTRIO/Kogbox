// Dependencies
use std::time::Duration;
use std::thread;

// Run Script Commands
pub fn script_command(code: String) {
    thread::sleep(Duration::from_millis(250));
    let code: &str = &code;
    let code = code.split("::");
    let code: Vec<&str> = code.collect();
    match code[0] {
        "test" => {
            match code[1] {
                "amogus" => println!("[Pog]: SUSSY!!!!"),
                "bruh" => println!("[Pog]: epic"),
                _ => invalid_argument(code[0], code[1])
            }
        }
        "harvest" => {
            match code[1] {
                "tree" => println!("[Task]: Harvested 1 tree"),
                _ => invalid_argument(code[0], code[1])
            }
        }
        _ => println!("[Err]: Invalid command, skipping (command: {})", code[0])
    }
}

// Invalid Argument
fn invalid_argument(command: &str, argument: &str) {
    println!("[Err]: Invalid argument {} for command {}, skipping", argument, command)
}