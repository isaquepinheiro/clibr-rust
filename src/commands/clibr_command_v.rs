use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use super::core::clibr_utils::utils;

#[derive(Default)]
pub struct CommandVersion {}

impl CommandVersion {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandVersion {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        print::print_version(&utils::version());
        true
    }
}
