use super::super::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print_version;

pub struct CommandVersion {}

impl CommandVersion {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandVersion {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &Box<dyn ICli>) -> bool {
        print_version("Version 0.1.0");
        true
    }
}
