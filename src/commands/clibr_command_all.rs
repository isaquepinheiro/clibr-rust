use super::core::clibr_command_pair::CommandPair;
use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use std::fs;
use std::rc::Rc;

#[derive(Default)]
pub struct CommandAll {}

impl ICommand for CommandAll {
    fn execute(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) -> bool {
        if file_name.is_empty() {
            print::print_alert("Invalid parameters!");
            return false;
        }

        let mut all_path: String = dir_name.to_string();
        let mut source_path: String = dir_name.to_string();

        if all_path.is_empty() || all_path == "." {
            all_path = "./src/".to_string();
            source_path = "./src".to_string();
        }

        if fs::metadata(&all_path).is_err() && fs::create_dir_all(&all_path).is_err() {
            print::print_alert("Failed to create directory!");
            return false;
        }

        source_path += &format!("/modules/{}", file_name);

        let is_horse: &bool = cli.get_tags().get("--horse").unwrap();
        if *is_horse {
            self.create_route_handle_horse(&source_path, file_name, cli);
        } else {
            self.create_route_handle(&source_path, file_name, cli);
        }

        self.create_module(&source_path, file_name, cli);
        self.create_controller(&format!("{}/controllers", source_path), file_name, cli);
        self.create_service(&format!("{}/services", source_path), file_name, cli);

        true
    }
}

impl CommandAll {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_module(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let command_pair: Option<&Rc<CommandPair>> =
            cli.get_commands().get("g").and_then(|map| map.get("m"));
        if let Some(command_pair) = command_pair {
            let command: Rc<dyn ICommand> = command_pair.get_command();
            let _is_success: bool = command.execute(dir_name, file_name, cli);
        }
    }

    pub fn create_controller(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let command_pair: Option<&Rc<CommandPair>> =
            cli.get_commands().get("g").and_then(|map| map.get("c"));
        if let Some(command_pair) = command_pair {
            let command: Rc<dyn ICommand> = command_pair.get_command();
            let _is_success: bool = command.execute(dir_name, file_name, cli);
        }
    }

    pub fn create_service(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let command_pair: Option<&Rc<CommandPair>> =
            cli.get_commands().get("g").and_then(|map| map.get("s"));
        if let Some(command_pair) = command_pair {
            let command: Rc<dyn ICommand> = command_pair.get_command();
            let _is_success: bool = command.execute(dir_name, file_name, cli);
        }
    }

    pub fn create_route_handle_horse(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let command_pair: Option<&Rc<CommandPair>> =
            cli.get_options_internal().get("horse-handler");
        if let Some(command_pair) = command_pair {
            let command: Rc<dyn ICommand> = command_pair.get_command();
            let _is_success: bool = command.execute(dir_name, file_name, cli);
        }
    }

    pub fn create_route_handle(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let command_pair: Option<&Rc<CommandPair>> = cli.get_options_internal().get("handler");
        if let Some(command_pair) = command_pair {
            let command: Rc<dyn ICommand> = command_pair.get_command();
            let _is_success: bool = command.execute(dir_name, file_name, cli);
        }
    }
}
