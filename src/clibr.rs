use clibr_main::main_cli;
use std::env;

pub mod clibr_main;
pub mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_cli(args);
}
