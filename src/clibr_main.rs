use crate::clibr_cli::Cli;
use crate::clibr_interfaces::{ICli, ICommand};
use crate::commands::core::clibr_print::print_alert;
use crate::commands::core::clibr_typedefs::{ListOptions, MapOptions};
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

pub fn main_cli(argv: Vec<String>) {
    let path_app: PathBuf = std::env::current_exe().unwrap();
    let path_cli: &Path = path_app.parent().unwrap();
    let path_formatted: String = format!("{}/templates", path_cli.display());
    let path_formatted: String = path_formatted.replace("\\", "/");
    let mut dir_name: String = String::new();
    let mut file_name: String = String::new();
    let mut command_key: String = String::new();
    let mut is_success: bool = false;
    let mut options: ListOptions = ListOptions::new();
    let mut cli: Box<dyn ICli> = Box::new(Cli::new(path_formatted));

    // Command find
    for arg in argv.iter().skip(1) {
        if cli.commands().contains_key(arg) {
            command_key = arg.to_string();
            cli.command_executed(arg.to_string());
            break;
        }
    }

    // Arguments find
    for arg in argv.iter().skip(1) {
        let options_find: &MapOptions = cli.commands_key(&command_key);

        if options_find.contains_key(arg) {
            options.push(arg.to_string());
        } else if cli.tags().contains_key(arg) {
            cli.set_tag_value(arg.to_string(), true);
        } else {
            if let Some(pos) = argv.iter().position(|item| item == arg) {
                let file_path: PathBuf = PathBuf::from(&argv[pos]);
                dir_name = file_path.parent().unwrap().to_str().unwrap().to_string();
                file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
            }
            if dir_name.is_empty() {
                dir_name = ".".to_string();
            }
        }
    }

    // Execute
    for item in options.into_iter() {
        let options_find: &MapOptions = cli.commands_key(&command_key);

        if !options_find.contains_key(&item) {
            continue;
        }

        let command: &Rc<dyn ICommand> = options_find[&item].get_command();
        is_success = command.execute(&dir_name, &file_name, &cli);
        if is_success {
            break;
        }
    }

    if is_success && !cli.updates().is_empty() {
        //let update_execute = CommandUpdateDpr::new();
        //is_success = update_execute.execute(&dir_name, &file_name, &cli);
    }

    if !is_success {
        print_alert("Run \'clibr --help\' for usage.");
    }
}
