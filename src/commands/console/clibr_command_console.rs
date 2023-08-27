use super::super::core::clibr_interfaces::{ICli, ICommand};
use super::super::core::clibr_print::print;
use super::super::core::clibr_utils::utils;

#[derive(Default)]
pub struct CommandGenerateProjectConsole {}

impl CommandGenerateProjectConsole {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandGenerateProjectConsole {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        let unit_name: String = _file_name.to_ascii_lowercase();
        let camel_case_name: String = format!("{}{}", _file_name[0..1].to_uppercase(), &_file_name[1..]);
        let program_name: String = utils::regex_replace_all(&camel_case_name, "-", "_");
        let template_file_path: String = format!("{}/console.project.pas", _cli.get_path_temp());
        let template_file_name: String = format!("{}/{}.dpr", _dir_name, unit_name);
        let template_content: String = utils::read_from_file(&template_file_path).unwrap_or_default();
        let modified_content: String = template_content.replace("{programName}", &program_name);

        let is_success = utils::write_to_file(&template_file_name, &modified_content);
        if is_success.is_ok() {
            print::print_create("CREATE",
                                 &template_file_name,
                              &utils::get_size_file(&template_file_name).unwrap_or_default(),
            );
        }
        is_success.is_ok()
    }
}
