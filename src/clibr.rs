use std::env;

mod clibr_main;
use clibr_main::main_cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_cli(0, args);
}