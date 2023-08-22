//use std::path::PathBuf;
//use std::collections::HashMap;
//use std::fs;
//use std::rc::Rc;

use crate::commands::core::clibr_print::print_alert;

pub fn main_cli(argc: i32, argv: Vec<String>) {
/*
    let executable_path = std::env::current_exe().unwrap();
    let path_cli = executable_path.parent().unwrap();
    let path_formatted = format!("{}/templates", path_cli.display());
    let path_formatted = path_formatted.replace("\\", "/");
    let mut dir_name = String::new();
    let mut file_name = String::new();
    let cli = Rc::new(Cli::new(&path_formatted).unwrap());
    let mut options = ListOptions::new();
    let mut command_options = HashMap::new();
    let mut is_success = false;

    // Command find
    for argl in 1..argc {
        if cli.commands().contains(argv[argl]) {
            command_options = cli.commands().at(argv[argl]);
            cli.command_executed(argv[argl]);
            break;
        }
    }

    // Arguments find
    for argl in 1..argc {
        let item = argv[argl].to_string();
        if command_options.contains(&item) {
            options.push_back(item);
        } else if cli.tags().contains(&item) {
            cli.set_tag_value(&item, true);
        } else {
            let file_path = PathBuf::from(&argv[argl]);
            dir_name = file_path.parent().unwrap().to_str().unwrap().to_string();
            file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
            if dir_name.is_empty() {
                dir_name = ".".to_string();
            }
        }
    }

    // Execute
    for item in options {
        if command_options.contains(&item) && command_options[&item].is_none() {
            continue;
        }

        let command = command_options[&item].get_command();
        is_success = command.execute(&dir_name, &file_name, &cli);
        if is_success {
            break;
        }
    }

    if is_success && !cli.updates().is_empty() {
        let update_execute = CommandUpdateDpr::new();
        is_success = update_execute.execute(&dir_name, &file_name, &cli);
    }

    if !is_success {
        crate::commands::core::clibr_print::print_alert("Run \'clibr --help\' for usage.");
    }
    options.clear();
    command_options.clear();
*/    
//print_alert("Run \'clibr --help\' for usage.");
}
