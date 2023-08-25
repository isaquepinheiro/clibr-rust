use super::super::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print_help;

pub struct CommandHelp {}

impl CommandHelp {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandHelp {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &Box<dyn ICli>) -> bool {
        print_help("CommandHelp");
        true
    }
}
