// Attribute
#![allow(non_snake_case)]
#![allow(static_mut_refs)]

// Imports
mod global;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

// Data structures
#[derive(Serialize, Deserialize, Clone)]
struct LoginEntry {
    title: String,
    username: String,
    password: String,
    url: String,
    notes: String,
}

#[derive(Serialize, Deserialize)]
struct PassFile {
    entries: HashMap<String, LoginEntry>,
}

fn main() {
    // Variables
    let mut isProgramRunning: bool = true;

    // Getting passfile location based on os
    match global::current_OS {
        "linux" => {
            let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
            let combined_path = format!("{}/.config/passforge/passfiles/", home_dir);
            unsafe { global::passfileLocation = Some(combined_path) }
        }
        "windows" => {
            let home_dir =
                env::var("USERPROFILE").expect("Failed to get USERPROFILE environment variable");
            let combined_path = format!("{}\\AppData\\Local\\passforge\\passfiles\\", home_dir);
            unsafe { global::passfileLocation = Some(combined_path) }
        }
        "macos" => {
            let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
            let combined_path = format!(
                "{}/Library/Application Support/passforge/passfiles/",
                home_dir
            );
            unsafe { global::passfileLocation = Some(combined_path) }
        }
        _ => println!("Unknown OS detected!"),
    }

    // Passfile selector
    PassfileSelector(&mut isProgramRunning);

    // Main program loop
    while isProgramRunning {
        ShowMainMenu(&mut isProgramRunning);
    }
}

// Custom functions
////////////////////
// Intro screen
fn IntroScreen() {
    // Clear terminal
    Command::new("clear")
        .spawn()
        .expect("Failed to clear screen");
    sleep(Duration::from_millis(200));

    // Display intro
    println!("###########################");
    println!("## Welcome to Passforge! ##");
    println!("##               V{}  ##", global::programVersion);
    println!("###########################");
    println!("");
}
// Passfile Selector
fn PassfileSelector(isProgramRunning: &mut bool) {
    // Variables
    let mut passFileOptionSelection: String = String::new();

    // Intro screen
    IntroScreen();
    sleep(Duration::from_secs(2));

    // Passfile options
    println!("----------------------------------");
    println!("Passfile Selector!");
    println!("1. Create a new passfile");
    println!("2. Open an existing passfile");
    println!("3. Open file from default location");
    println!("4. Exit");
    println!("----------------------------------");

    // Taking input
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut passFileOptionSelection)
        .expect("Failed to read line");
    let passFileOptionSelection = passFileOptionSelection.trim().to_string();

    // Decision on input
    match passFileOptionSelection.as_str() {
        "1" => CreateNewPassfile(),
        "2" => OpenExistingPassfile(),
        "3" => OpenDefaultPassfile(),
        "4" => {
            println!("Exiting the program!");
            *isProgramRunning = false;
        }
        _ => {
            println!("Invalid option!");
            PassfileSelector(isProgramRunning);
        }
    }
}

// Create new passfile
fn CreateNewPassfile() {
    // Intro screen
    IntroScreen();

    println!("Creating a new passfile!");
    println!("----------------------------------");

    let mut filename = String::new();
    print!("Create passfile name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");
    let filename = filename.trim().to_string();

    let mut password = String::new();
    print!("Create encryption password: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    // Create empty PassFile structure
    let passfile = PassFile {
        entries: HashMap::new(),
    };

    // Convert to JSON
    let json_data = serde_json::to_string(&passfile).expect("Failed to serialize data");

    // Encrypt the JSON data
    let mc = new_magic_crypt!(&password, 256);
    let encrypted_data = mc.encrypt_str_to_base64(&json_data);

    // Create directory if it doesn't exist
    fs::create_dir_all(unsafe {
        global::passfileLocation
            .as_ref()
            .expect("Passfile location is None")
    })
    .expect("Failed to create directory");

    // Save to file
    let filepath = format!(
        "{}{}.pfg",
        unsafe {
            global::passfileLocation
                .as_ref()
                .expect("Passfile location is None")
        },
        filename
    );
    let mut file = File::create(&filepath).expect("Failed to create file");
    file.write_all(encrypted_data.as_bytes())
        .expect("Failed to write to file");

    println!("Passfile created successfully at {}", filepath);
    unsafe {
        global::current_passfile = Some(filepath);
        global::encryption_password = Some(password);
    }

    sleep(Duration::from_secs(2));
}

// Open existing passfile
fn OpenExistingPassfile() {
    // Intro screen
    IntroScreen();

    println!("Opening an existing passfile!");
    println!("----------------------------------");

    let mut filepath = String::new();
    print!("Enter passfile path: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut filepath)
        .expect("Failed to read line");
    let filepath = filepath.trim().to_string();

    if !Path::new(&filepath).exists() {
        println!("File does not exist!");
        return;
    }

    let mut password = String::new();
    print!("Enter encryption password: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    // Try to decrypt and open
    if let Ok(encrypted_data) = fs::read_to_string(&filepath) {
        let mc = new_magic_crypt!(&password, 256);
        match mc.decrypt_base64_to_string(&encrypted_data) {
            Ok(decrypted_data) => {
                // Try to parse JSON
                match serde_json::from_str::<PassFile>(&decrypted_data) {
                    Ok(_) => {
                        println!("Passfile opened successfully!");
                        unsafe {
                            global::current_passfile = Some(filepath);
                            global::encryption_password = Some(password);
                        }
                    }
                    Err(_) => println!("Invalid passfile format!"),
                }
            }
            Err(_) => println!("Incorrect password or corrupted file!"),
        }
    } else {
        println!("Failed to read file!");
    }
}

// Open default passfile
fn OpenDefaultPassfile() {
    // Intro screen
    IntroScreen();

    println!("Opening file from default location!");
    println!("--------------------------------------");

    let default_path = "passfiles/default.pfg";

    if !Path::new(default_path).exists() {
        println!("Default passfile does not exist! Creating one...");

        let mut password = String::new();
        print!("Enter encryption password for new default passfile: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");
        let password = password.trim().to_string();

        // Create empty PassFile structure
        let passfile = PassFile {
            entries: HashMap::new(),
        };

        // Convert to JSON
        let json_data = serde_json::to_string(&passfile).expect("Failed to serialize data");

        // Encrypt the JSON data
        let mc = new_magic_crypt!(&password, 256);
        let encrypted_data = mc.encrypt_str_to_base64(&json_data);

        // Create directory if it doesn't exist
        fs::create_dir_all("passfiles").expect("Failed to create directory");

        // Save to file
        let mut file = File::create(default_path).expect("Failed to create file");
        file.write_all(encrypted_data.as_bytes())
            .expect("Failed to write to file");

        println!("Default passfile created successfully!");
        unsafe {
            global::current_passfile = Some(default_path.to_string());
            global::encryption_password = Some(password);
        }
    } else {
        let mut password = String::new();
        print!("Enter encryption password: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");
        let password = password.trim().to_string();

        // Try to decrypt and open
        if let Ok(encrypted_data) = fs::read_to_string(default_path) {
            let mc = new_magic_crypt!(&password, 256);
            match mc.decrypt_base64_to_string(&encrypted_data) {
                Ok(decrypted_data) => {
                    // Try to parse JSON
                    match serde_json::from_str::<PassFile>(&decrypted_data) {
                        Ok(_) => {
                            println!("Default passfile opened successfully!");
                            unsafe {
                                global::current_passfile = Some(default_path.to_string());
                                global::encryption_password = Some(password);
                            }
                        }
                        Err(_) => println!("Invalid passfile format!"),
                    }
                }
                Err(_) => println!("Incorrect password or corrupted file!"),
            }
        } else {
            println!("Failed to read default passfile!");
        }
    }
}

// Show main menu
fn ShowMainMenu(isProgramRunning: &mut bool) {
    // Intro screen
    IntroScreen();

    if unsafe { global::current_passfile.is_none() } {
        println!("No passfile is open. Please open or create a passfile first.");
        PassfileSelector(isProgramRunning);
        return;
    }

    // Variables
    let mut mainMenuSelection: String = String::new();

    // Main menu options
    println!("\n----------------------------------");
    println!("Main Menu - {}", unsafe {
        global::current_passfile.as_ref().unwrap()
    });
    println!("1. Add new login");
    println!("2. Show all logins");
    println!("3. Generate password");
    println!("4. Change passfile");
    println!("5. Exit");
    println!("----------------------------------");

    // Taking input
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut mainMenuSelection)
        .expect("Failed to read line");
    let mainMenuSelection = mainMenuSelection.trim().to_string();

    // Decision on input
    match mainMenuSelection.as_str() {
        "1" => AddNewLogin(),
        "2" => ShowAllLogins(),
        "3" => GeneratePassword(),
        "4" => {
            SaveCurrentPassfile();
            PassfileSelector(isProgramRunning);
        }
        "5" => {
            SaveCurrentPassfile();
            println!("Exiting the program!");
            *isProgramRunning = false;
        }
        _ => {
            println!("Invalid option!");
        }
    }
}

// Add new login
fn AddNewLogin() {
    // Intro screen
    IntroScreen();

    println!("\nAdding a new login entry:");
    println!("----------------------------------");

    let mut title = String::new();
    print!("Enter title: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let title = title.trim().to_string();

    let mut username = String::new();
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    let mut password = String::new();
    print!("Enter password (or type 'generate' to generate one): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    let password = if password == "generate" {
        let generated = generate_random_password(16);
        println!("Generated password: {}", generated);
        generated
    } else {
        password
    };

    let mut url = String::new();
    print!("Enter URL: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    let url = url.trim().to_string();

    let mut notes = String::new();
    print!("Enter notes: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut notes)
        .expect("Failed to read line");
    let notes = notes.trim().to_string();

    // Create entry
    let entry = LoginEntry {
        title: title.clone(),
        username,
        password,
        url,
        notes,
    };

    // Load current passfile
    if let Some(filepath) = unsafe { &global::current_passfile } {
        if let Some(password) = unsafe { &global::encryption_password } {
            if let Ok(encrypted_data) = fs::read_to_string(filepath) {
                let mc = new_magic_crypt!(password, 256);
                if let Ok(decrypted_data) = mc.decrypt_base64_to_string(&encrypted_data) {
                    if let Ok(mut passfile) = serde_json::from_str::<PassFile>(&decrypted_data) {
                        // Add new entry
                        passfile.entries.insert(title, entry);

                        // Save updated passfile
                        let json_data =
                            serde_json::to_string(&passfile).expect("Failed to serialize data");
                        let encrypted_data = mc.encrypt_str_to_base64(&json_data);

                        let mut file = File::create(filepath).expect("Failed to open file");
                        file.write_all(encrypted_data.as_bytes())
                            .expect("Failed to write to file");

                        println!("Login entry added successfully!");
                        return;
                    }
                }
            }
        }
    }

    println!("Failed to add login entry!");
}

// Show all logins
fn ShowAllLogins() {
    // Intro screen
    IntroScreen();

    if let Some(filepath) = unsafe { &global::current_passfile } {
        if let Some(password) = unsafe { &global::encryption_password } {
            if let Ok(encrypted_data) = fs::read_to_string(filepath) {
                let mc = new_magic_crypt!(password, 256);
                if let Ok(decrypted_data) = mc.decrypt_base64_to_string(&encrypted_data) {
                    if let Ok(passfile) = serde_json::from_str::<PassFile>(&decrypted_data) {
                        println!("\n------ Stored Logins ------");

                        if passfile.entries.is_empty() {
                            println!("");
                            println!("No entries found!");
                            sleep(Duration::from_secs(2));
                            return;
                        }

                        let mut index = 1;
                        let titles: Vec<String> = passfile.entries.keys().cloned().collect();

                        for title in &titles {
                            println!("{}. {}", index, title);
                            index += 1;
                        }

                        let mut selection = String::new();
                        print!("\nEnter number to view details (or 0 to go back): ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut selection)
                            .expect("Failed to read line");

                        if let Ok(num) = selection.trim().parse::<usize>() {
                            if num == 0 {
                                return;
                            }

                            if num > 0 && num <= titles.len() {
                                let title = &titles[num - 1];
                                if let Some(entry) = passfile.entries.get(title) {
                                    // Intro screen
                                    IntroScreen();

                                    println!("\n------ {} ------", entry.title);
                                    println!("Username: {}", entry.username);
                                    println!("Password: {}", entry.password);
                                    println!("URL: {}", entry.url);
                                    println!("Notes: {}", entry.notes);

                                    println!("\nOptions:");
                                    println!("1. Copy password to clipboard");
                                    println!("2. Edit entry");
                                    println!("3. Delete entry");
                                    println!("4. Go back");

                                    let mut option = String::new();
                                    print!("Enter option: ");
                                    io::stdout().flush().unwrap();
                                    io::stdin()
                                        .read_line(&mut option)
                                        .expect("Failed to read line");

                                    match option.trim() {
                                        "1" => {
                                            // Copy to clipboard functionality would require additional crates
                                            println!(
                                                "Clipboard copy function is under development!"
                                            );
                                            sleep(Duration::from_secs(2));
                                        }
                                        "2" => {
                                            EditLoginEntry(&title);
                                        }
                                        "3" => {
                                            DeleteLoginEntry(&title);
                                        }
                                        _ => {}
                                    }
                                }
                            } else {
                                println!("Invalid selection!");
                            }
                        }
                        return;
                    }
                }
            }
        }
    }

    println!("Failed to load login entries!");
}

// Edit login entry
fn EditLoginEntry(title: &str) {
    if let Some(filepath) = unsafe { &global::current_passfile } {
        if let Some(password) = unsafe { &global::encryption_password } {
            if let Ok(encrypted_data) = fs::read_to_string(filepath) {
                let mc = new_magic_crypt!(password, 256);
                if let Ok(decrypted_data) = mc.decrypt_base64_to_string(&encrypted_data) {
                    if let Ok(mut passfile) = serde_json::from_str::<PassFile>(&decrypted_data) {
                        if let Some(entry) = passfile.entries.get(title).cloned() {
                            println!("----------------------------------");
                            println!("\nEditing entry: {}", title);
                            println!("(Press Enter to keep current value)");

                            let mut new_username = String::new();
                            print!("Username [{}]: ", entry.username);
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut new_username)
                                .expect("Failed to read line");
                            let new_username = if new_username.trim().is_empty() {
                                entry.username
                            } else {
                                new_username.trim().to_string()
                            };

                            let mut new_password = String::new();
                            print!("Password [{}]: ", entry.password);
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut new_password)
                                .expect("Failed to read line");
                            let new_password = if new_password.trim().is_empty() {
                                entry.password
                            } else if new_password.trim() == "generate" {
                                let generated = generate_random_password(16);
                                println!("Generated password: {}", generated);
                                generated
                            } else {
                                new_password.trim().to_string()
                            };

                            let mut new_url = String::new();
                            print!("URL [{}]: ", entry.url);
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut new_url)
                                .expect("Failed to read line");
                            let new_url = if new_url.trim().is_empty() {
                                entry.url
                            } else {
                                new_url.trim().to_string()
                            };

                            let mut new_notes = String::new();
                            print!("Notes [{}]: ", entry.notes);
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut new_notes)
                                .expect("Failed to read line");
                            let new_notes = if new_notes.trim().is_empty() {
                                entry.notes
                            } else {
                                new_notes.trim().to_string()
                            };

                            // Create updated entry
                            let updated_entry = LoginEntry {
                                title: title.to_string(),
                                username: new_username,
                                password: new_password,
                                url: new_url,
                                notes: new_notes,
                            };

                            // Update entry
                            passfile.entries.insert(title.to_string(), updated_entry);

                            // Save updated passfile
                            let json_data =
                                serde_json::to_string(&passfile).expect("Failed to serialize data");
                            let encrypted_data = mc.encrypt_str_to_base64(&json_data);

                            let mut file = File::create(filepath).expect("Failed to open file");
                            file.write_all(encrypted_data.as_bytes())
                                .expect("Failed to write to file");

                            println!("Login entry updated successfully!");
                            return;
                        }
                    }
                }
            }
        }
    }

    println!("Failed to update login entry!");
}

// Delete login entry
fn DeleteLoginEntry(title: &str) {
    if let Some(filepath) = unsafe { &global::current_passfile } {
        if let Some(password) = unsafe { &global::encryption_password } {
            if let Ok(encrypted_data) = fs::read_to_string(filepath) {
                let mc = new_magic_crypt!(password, 256);
                if let Ok(decrypted_data) = mc.decrypt_base64_to_string(&encrypted_data) {
                    if let Ok(mut passfile) = serde_json::from_str::<PassFile>(&decrypted_data) {
                        print!("Are you sure you want to delete '{}' (y/n)? ", title);
                        io::stdout().flush().unwrap();

                        let mut confirmation = String::new();
                        io::stdin()
                            .read_line(&mut confirmation)
                            .expect("Failed to read line");

                        if confirmation.trim().to_lowercase() == "y" {
                            passfile.entries.remove(title);

                            // Save updated passfile
                            let json_data =
                                serde_json::to_string(&passfile).expect("Failed to serialize data");
                            let encrypted_data = mc.encrypt_str_to_base64(&json_data);

                            let mut file = File::create(filepath).expect("Failed to open file");
                            file.write_all(encrypted_data.as_bytes())
                                .expect("Failed to write to file");

                            println!("Login entry deleted successfully!");
                            return;
                        } else {
                            println!("Deletion cancelled.");
                            return;
                        }
                    }
                }
            }
        }
    }

    println!("Failed to delete login entry!");
}

// Generate password
fn GeneratePassword() {
    // Intro screen
    IntroScreen();

    println!("\nPassword Generator");
    println!("----------------------------------");

    let mut length = String::new();
    print!("Enter password length (default: 16): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");

    let length: usize = match length.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => 16,
    };

    let mut include_special = String::new();
    print!("Include special characters? (y/n, default: y): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut include_special)
        .expect("Failed to read line");

    let include_special = include_special.trim().to_lowercase() != "n";

    let password = if include_special {
        generate_random_password_with_special(length)
    } else {
        generate_random_password(length)
    };

    println!("\nGenerated Password: {}", password);

    let mut save_option = String::new();
    print!("Save this password to a new entry? (y/n): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut save_option)
        .expect("Failed to read line");

    if save_option.trim().to_lowercase() == "y" {
        let mut title = String::new();
        print!("Enter title: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");
        let title = title.trim().to_string();

        let mut username = String::new();
        print!("Enter username: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
        let username = username.trim().to_string();

        let mut url = String::new();
        print!("Enter URL: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read line");
        let url = url.trim().to_string();

        let mut notes = String::new();
        print!("Enter notes: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut notes)
            .expect("Failed to read line");
        let notes = notes.trim().to_string();

        // Create entry
        let entry = LoginEntry {
            title: title.clone(),
            username,
            password,
            url,
            notes,
        };

        // Save entry
        if let Some(filepath) = unsafe { &global::current_passfile } {
            if let Some(encryption_password) = unsafe { &global::encryption_password } {
                if let Ok(encrypted_data) = fs::read_to_string(filepath) {
                    let mc = new_magic_crypt!(encryption_password, 256);
                    if let Ok(decrypted_data) = mc.decrypt_base64_to_string(&encrypted_data) {
                        if let Ok(mut passfile) = serde_json::from_str::<PassFile>(&decrypted_data)
                        {
                            // Add new entry
                            passfile.entries.insert(title, entry);

                            // Save updated passfile
                            let json_data =
                                serde_json::to_string(&passfile).expect("Failed to serialize data");
                            let encrypted_data = mc.encrypt_str_to_base64(&json_data);

                            let mut file = File::create(filepath).expect("Failed to open file");
                            file.write_all(encrypted_data.as_bytes())
                                .expect("Failed to write to file");

                            println!("Login entry with generated password added successfully!");
                            return;
                        }
                    }
                }
            }
            println!("Failed to save entry!");
        }
    }
}

// Helper functions
fn generate_random_password(length: usize) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| char::from(rng.sample(Alphanumeric)))
        .collect()
}

fn generate_random_password_with_special(length: usize) -> String {
    let mut rng = thread_rng();
    let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    let alphanumeric = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let all_chars = alphanumeric.to_string() + special_chars;
    let char_vec: Vec<char> = all_chars.chars().collect();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..char_vec.len());
            char_vec[idx]
        })
        .collect()
}

// Save current passfile before exit
fn SaveCurrentPassfile() {
    if unsafe { global::current_passfile.is_some() }
        && unsafe { global::encryption_password.is_some() }
    {
        println!("Saving and encrypting passfile...");
        // Actual saving happens in other functions, this just logs the action
    }
}
