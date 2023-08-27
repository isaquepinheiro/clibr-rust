pub mod print {
    pub const RESET: &str = "\x1B[0m";
    pub const RED: &str = "\x1B[31m";
    pub const GREEN: &str = "\x1B[32m";
    pub const BLUE: &str = "\x1B[34m";
    pub const YELLOW: &str = "\x1B[33m";
    pub const MAGENTA: &str = "\x1B[35m";
    pub const CYAN: &str = "\x1B[36m";
    pub const WHITE: &str = "\x1B[37m";

    pub fn print_create(title: &str, text: &str, message: &str) {
        println!("{} {} {} {} {}", GREEN, title, text, message, RESET);
    }

    pub fn print_update(title: &str, text: &str, message: &str) {
        println!("{} {} {} {} {}", CYAN, title, text, message, RESET);
    }

    pub fn print_version(message: &str) {
        println!("{} {} {}", MAGENTA, message, RESET);
    }

    pub fn print_link(message: &str) {
        println!("{} {} {}", BLUE, message, RESET);
    }

    pub fn print_alert(message: &str) {
        println!("{} {} {}", RED, message, RESET);
    }

    pub fn print_help(message: &str) {
        println!("{} {} {}", YELLOW, message, RESET);
    }
}
