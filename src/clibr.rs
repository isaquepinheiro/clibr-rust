use clibr_main::main_cli;
use std::env;

mod clibr_cli;
mod clibr_interfaces;
mod clibr_main;
mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_cli(args);
}
