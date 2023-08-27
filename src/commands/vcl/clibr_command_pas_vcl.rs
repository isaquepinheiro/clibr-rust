use crate::commands::core::clibr_print::print;
use crate::commands::core::clibr_utils::utils;
use super::super::core::clibr_interfaces::{ICli, ICommand};

#[derive(Default)]
pub struct CommandGenerateUnitVCL {}

impl CommandGenerateUnitVCL {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandGenerateUnitVCL {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        if _file_name.is_empty() {
            print::print_alert("Invalid parameters!");
            return false;
        }
        let unit_name: String = _file_name.to_lowercase();
        let template_file_path: String = format!("{}/vcl.project.unit.pas", _cli.get_path_temp());
        let template_file_name: String = format!("{}/u{}.pas", _dir_name, unit_name);
        let template_content: String = utils::read_from_file(&template_file_path).unwrap_or_default();
        let modified_content: String = template_content.to_string().replace("{unitName}", &unit_name);
        let is_success = utils::write_to_file(&template_file_name, &modified_content);
        if is_success.is_ok() {
            print::print_create("CREATE", 
                                 &template_file_name, 
                              &utils::get_size_file(&template_file_name).unwrap_or_default());
        }
        is_success.is_ok()
    }
}
