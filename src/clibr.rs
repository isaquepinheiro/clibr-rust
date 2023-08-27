use std::env;
use clibr_main::main_cli;

pub mod commands;
pub mod clibr_main;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_cli(args);
}
