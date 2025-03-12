// Attribute
#![allow(non_snake_case)]

// Imports
mod global;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Variables
    let mut isProgramRunning: bool = true;

    // Title of the project
    println!("###########################");
    println!("## Welcome to Passforge! ##");
    println!("##               V{}  ##", global::programVersion);
    println!("###########################");
    println!("");
    sleep(Duration::from_secs(2));

    // Passfile selector
    PassfileSelector(&mut isProgramRunning);    

    // Main program loop
    while isProgramRunning {}
}

// Custom functions
////////////////////
// Passfile Selector
fn PassfileSelector(isProgramRunning: &mut bool) {
    // Variables
    let mut passFileOptionSelection: String = String::new();

    // Passfile options
    println!("----------------------------------");
    println!("Passfile Selector!");
    println!("1. Create a new passfile");
    println!("2. Open an existing passfile");
    println!("3. Open file from default location");
    println!("4. Exit");
    println!("----------------------------------");
    
    // Taking inpur
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut passFileOptionSelection)
        .expect("Failed to read line");
    let passFileOptionSelection = passFileOptionSelection.trim().to_string();

    // Decision on input
    match passFileOptionSelection.as_str() {
        "1" => CreateNewPassfile(),
        "2" => {
            println!("Opening an existing passfile!");
        }
        "3" => {
            println!("Opening file from default location!");
        }
        "4" => {
            println!("Exiting the program!");
            *isProgramRunning = false;
        }
        _ => {
            println!("Invalid option!");
        }
    }
}

// Create new passfile
fn CreateNewPassfile() {}