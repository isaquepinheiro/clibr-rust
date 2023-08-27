use super::console::clibr_command_console::CommandGenerateProjectConsole;
use super::core::clibr_command_pair::CommandPair;
use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use super::core::clibr_typedefs::MapTags;
use super::vcl::clibr_command_dfm_vcl::CommandGenerateFormVCL;
use super::vcl::clibr_command_pas_vcl::CommandGenerateUnitVCL;
use std::fs;
use std::rc::Rc;

#[derive(Default)]
pub struct CommandGenerateProject {}

impl ICommand for CommandGenerateProject {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        if _file_name.is_empty() {
            print::print_alert("Invalid parameters!");
            return false;
        }

        if fs::metadata(_dir_name).is_err() && fs::create_dir_all(_dir_name).is_err() {
            print::print_alert("Failed to create directory!");
            return false;
        }

        let project_path: String = _dir_name.to_string();
        let mut source_path: String = _dir_name.to_string();
        source_path.push_str("/src/modules/ping");

        let tags: &MapTags = _cli.get_tags();
        // VCL
        let is_vcl: &bool = tags.get("--vcl").unwrap();
        // Horse
        let is_horse: &bool = tags.get("--horse").unwrap();

        if *is_horse {
            self.create_project_horse(&project_path, _file_name, _cli);
            self.create_app_module(&format!("{}/src", project_path), "app", _cli);
            self.create_route_handle_horse(&source_path, "ping", _cli);
        } else if *is_vcl {
            self.create_project_vcl(&project_path, _file_name, _cli);
            self.create_app_module(&format!("{}/src", project_path), "app", _cli);
            self.create_route_handle(&source_path, "ping", _cli);
        } else {
            self.create_project(&project_path, _file_name, _cli);
            self.create_app_module(&format!("{}/src", project_path), "app", _cli);
            self.create_route_handle(&source_path, "ping", _cli);
        }

        self.create_module(&source_path, "ping", _cli);
        self.create_controller(&format!("{}/controllers", source_path), "ping", _cli);
        self.create_service(&format!("{}/services", source_path), "ping", _cli);

        true
    }
}

impl CommandGenerateProject {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_project(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let project_console: CommandGenerateProjectConsole = CommandGenerateProjectConsole::new();
        let _is_success: bool = project_console.execute(dir_name, file_name, cli);
    }

    pub fn create_app_module(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        self.create_module(dir_name, file_name, cli);
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

    pub fn create_project_horse(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let command_pair: Option<&Rc<CommandPair>> = cli.get_options_internal().get("horse-app");
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

    pub fn create_project_vcl(&self, dir_name: &str, file_name: &str, cli: &mut dyn ICli) {
        let form_vcl: CommandGenerateFormVCL = CommandGenerateFormVCL::new();
        let unit_vcl: CommandGenerateUnitVCL = CommandGenerateUnitVCL::new();
        let command_pair: &Rc<CommandPair> = cli.get_options_internal().get("vcl-app").unwrap();
        let command: Rc<dyn ICommand> = command_pair.get_command();

        let _is_success1: bool = command.execute(dir_name, file_name, cli);
        let _is_success2: bool = form_vcl.execute(dir_name, file_name, cli);
        let _is_success3: bool = unit_vcl.execute(dir_name, file_name, cli);
    }
}
