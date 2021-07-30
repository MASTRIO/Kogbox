// Dependencies
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