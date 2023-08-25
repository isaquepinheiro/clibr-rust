use crate::commands::core::clibr_typedefs::{ListUpdates, MapCommands, MapOptions, MapTags};

/// Base trait for Command-Line Interface (CLI) functionality.
pub trait ICli {
    fn path_temp(&self) -> &String;
    fn commands(&self) -> &MapCommands;
    fn commands_key(&self, key: &String) -> &MapOptions;
    fn options_internal(&self) -> &MapOptions;
    fn tags(&self) -> &MapTags;
    fn updates(&self) -> &ListUpdates;
    fn command_executed(&mut self, value: String);
    fn set_tag_value(&mut self, name: String, value: bool);
    fn set_update(&mut self, value: &String);
}

/// Base trait for executable commands.
pub trait ICommand {
    fn execute(&self, dir_name: &str, file_name: &str, cli: &Box<dyn ICli>) -> bool;
}
