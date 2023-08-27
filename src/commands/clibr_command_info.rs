use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use super::core::clibr_typedefs::ListText;
use super::core::clibr_utils::utils;
use std::vec;

#[derive(Default)]
pub struct CommandInfo {}

impl CommandInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandInfo {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        let print_text: ListText = vec![
            "".to_string(),
            " ______   ____    ____  ________  ______              ______  _____     _____  "
                .to_string(),
            "|_   _ `. |_  \\  /   _||_   __  ||_   _ \\           .\' ___  ||_   _|   |_   _| "
                .to_string(),
            "  | | `. \\ |   \\/   |    | |_ \\_|  | |_) | _ .--.  / .\'   \\_|  | |       | |   "
                .to_string(),
            "  | |  | | | |\\  /| |    |  _|     |  __\'.[ `/\'`\\] | |         | |   _   | |   "
                .to_string(),
            " _| |_.\' /_| |_\\/_| |_  _| |_     _| |__) || |     \\ `.___.\'\\ _| |__/ | _| |_  "
                .to_string(),
            "|______.\'|_____||_____||_____|   |_______/[___]     `.____ .\'|________||_____| "
                .to_string(),
            "".to_string(),
            format!("  Version: {}", &utils::version()),
            "  Author: Isaque Pinheiro".to_string(),
            "  Email: isaquesp@gmail.com".to_string(),
            "  Github: https://github.com/HashLoad/DMFBr".to_string(),
            "  Documentation: https://dmfbr-en.docs-br.com".to_string(),
            "".to_string(),
        ];
        print::print_help(&print_text.join("\n"));
        true
    }
}
