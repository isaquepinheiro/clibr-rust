use super::super::core::clibr_interfaces::{ICli, ICommand};
use super::super::core::clibr_print::print;
use super::super::core::clibr_utils::utils;

#[derive(Default)]
pub struct CommandGenerateFormVCL {}

impl CommandGenerateFormVCL {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandGenerateFormVCL {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        let unit_name: String = _file_name.to_ascii_lowercase();
        let template_file_path: String = format!("{}/vcl.project.form.pas", _cli.get_path_temp());
        let template_file_name: String = format!("{}/u{}.dfm", _dir_name, &unit_name);
        let template_content: String = utils::read_from_file(&template_file_path).unwrap_or_default();

        if _file_name.is_empty() {
            print::print_alert("Invalid parameters!");
            return false;
        }

        let is_success = utils::write_to_file(&template_file_name, &template_content);
        if is_success.is_ok() {
            print::print_create("CREATE",
                                 &template_file_name,
                              &utils::get_size_file(&template_file_name).unwrap_or_default(),
            );
        }
        is_success.is_ok()
    }
}
