// Attribute
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// Imports
use std::env;

// Global variables for Passforge
pub static current_OS: &str = env::consts::OS;
pub static mut programStartedOnce: bool = false;

pub static mut passfileLocation: Option<String> = None;

// Program version
pub static programVersion: &str = "1.4.3";

// Global application state
pub static mut current_passfile: Option<String> = None;
pub static mut encryption_password: Option<String> = None;

// Safe accessor functions for global state (to handle unsafe properly)
fn get_current_passfile() -> Option<String> {
    unsafe {
        current_passfile.clone()
    }
}

pub fn set_current_passfile(path: Option<String>) {
    unsafe {
        current_passfile = path;
    }
}

pub fn get_encryption_password() -> Option<String> {
    unsafe {
        encryption_password.clone()
    }
}

pub fn set_encryption_password(password: Option<String>) {
    unsafe {
        encryption_password = password;
    }
}