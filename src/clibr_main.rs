use crate::commands::core::clibr_cli::Cli;
use crate::commands::core::clibr_interfaces::{ICli, ICommand};
use crate::commands::core::clibr_print::print;
use crate::commands::core::clibr_typedefs::{ListOptions, MapOptions};
use crate::commands::update::clibr_update_dpr::CommandUpdateDpr;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

pub fn main_cli(argv: Vec<String>) {
    let path_app: PathBuf = std::env::current_exe().unwrap();
    let path_cli: &Path = path_app.parent().unwrap();
    let path_formatted: String = format!("{}/templates", path_cli.display());
    let path_formatted: String = path_formatted.replace('\\', "/");
    let mut dir_name: String = String::new();
    let mut file_name: String = String::new();
    let mut command_key: String = String::new();
    let mut is_success: bool = false;
    let mut options: ListOptions = ListOptions::new();
    let mut cli: Cli = Cli::new(path_formatted);

    // Command find
    for arg in argv.iter().skip(1) {
        if !cli.get_commands().contains_key(arg) {
            continue;
        }
        command_key = arg.to_string();
        cli.set_command_executed(arg.to_string());
        break;
    }

    // Arguments find
    for arg in argv.iter().skip(1) {
        let options_find: &MapOptions = cli.get_commands_key(&command_key);

        if options_find.contains_key(arg) {
            options.push(arg.to_string());
        } else if cli.get_tags().contains_key(arg) {
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
        let options_find: &MapOptions = cli.get_commands_key(&command_key);

        match options_find.get(&item) {
            Some(command_pair) => {
                let command: Rc<dyn ICommand> = command_pair.get_command();

                if command.execute(&dir_name, &file_name, &mut cli) {
                    is_success = true;
                    break;
                }
            }
            None => continue,
        }
    }

    // Update DPR
    if is_success && !cli.get_updates().is_empty() {
        let update_execute = CommandUpdateDpr::new();
        is_success = update_execute.execute(&dir_name, &file_name, &mut cli);
    }

    if !is_success {
        print::print_alert("Run \'clibr --help\' for usage.");
    }
}
