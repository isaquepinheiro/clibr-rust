use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use super::core::clibr_utils::utils;
use std::fs;

#[derive(Default)]
pub struct CommandService {}

impl CommandService {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandService {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        if _file_name.is_empty() {
            print::print_alert("Invalid parameters!");
            return false;
        }
        let unit_name: String = {_file_name.to_lowercase()};
        let camel_casename: String = format!("{}{}", _file_name[0..1].to_uppercase(), &_file_name[1..]);
        let class_name: String = format!("T{}Service", &camel_casename);
        let mut source_path: String = {_dir_name.to_string()};

        if source_path.is_empty() || source_path == "." {
            source_path = "./".to_string();
        }

        if fs::metadata(&source_path).is_err() && fs::create_dir_all(&source_path).is_err() {
            print::print_alert("Failed to create directory!");
            return false;
        }
        
        let template_file_path: String = format!("{}/service.pas", _cli.get_path_temp());
        let template_file_name: String = format!("{}/{}.service.pas", &source_path, &unit_name);
        let template_content: String = utils::read_from_file(&template_file_path).unwrap_or_default();
        let modified_content: String = template_content.to_string()
            .replace("{unitName}", &unit_name)
            .replace("{serviceName}", &class_name);
        
        if fs::write(&template_file_name, modified_content).is_err() {
            print::print_alert("Failed to write modified content to file!");
            return false;
        }
        
        print::print_create("CREATE", 
                            &template_file_name, 
                            &utils::get_size_file(&template_file_name).unwrap_or_default());

        // List Update DPR
        let update: String = format!(
            "  {}.service in 'src\\modules\\{}\\{}.service.pas',",
            &unit_name, &_file_name, &unit_name
        );
        _cli.set_update(update);
        
        true
    }
}
