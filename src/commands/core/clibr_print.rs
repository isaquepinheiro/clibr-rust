use std::io::{self, Write};

pub const RESET: &'static str = "\\u001b[0m";
pub const RED: &'static str = "\\u001b[31m";
pub const GREEN: &'static str = "\\u001b[32m";
pub const BLUE: &'static str = "\\u001b[34m";
pub const YELLOW: &'static str = "\\u001b[33m";
pub const MAGENTA: &'static str = "\\u001b[35m";
pub const CYAN: &'static str = "\\u001b[36m";
pub const WHITE: &'static str = "\\u001b[37m";

pub fn print_create(title: &str, text: &str, message: &str) {
    println!("{} {} {} {}", Self::GREEN, title, text, message, Self::RESET);
}

pub fn print_update(title: &str, text: &str, message: &str) {
    println!("{} {} {} {} {}", Self::CYAN, title, text, message, Self::RESET);
}

pub fn print_version(message: &str) {
    println!("{} {} {}", Self::MAGENTA, message, Self::RESET);
}

pub fn print_link(message: &str) {
    println!("{} {} {}", Self::BLUE, message, Self::RESET);
}

pub fn print_alert(message: &str) {
    println!("{} {} {}", Self::RED, message, Self::RESET);
}

pub fn print_help(message: &str) {
    println!("{} {} {}", Self::YELLOW, message, Self::RESET);
}
