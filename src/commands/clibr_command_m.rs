use super::core::clibr_ifthen::IfThen;
use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use super::core::clibr_utils::utils;
use std::fs;

#[derive(Default)]
pub struct CommandModule {}

impl ICommand for CommandModule {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        if _file_name.is_empty() {
            print::print_alert("Invalid parameters!");
            return false;
        }
        let unit_name: String = {_file_name.to_lowercase()};
        let camel_casename: String = format!("{}{}", _file_name[0..1].to_uppercase(), &_file_name[1..]);
        let class_name: String = format!("T{}Module", &camel_casename);
        let is_app_module: bool = {&unit_name == "app"};
        let mut source_path: String = {_dir_name.to_string()};

        if source_path.is_empty() || source_path == "." {
            source_path = format!("./src/modules/{}", _file_name).to_string();
        }

        if fs::metadata(&source_path).is_err() && fs::create_dir_all(&source_path).is_err() {
            print::print_alert("Failed to create directory!");
            return false;
        }
        
        let template_file_path: String = IfThen::<String>::new().when(is_app_module)
                                                                .then(format!("{}/module.app.pas", _cli.get_path_temp()))
                                                                .else_or(format!("{}/module.pas", _cli.get_path_temp()))
                                                                .evaluate();
        let template_file_name: String = format!("{}/{}module.pas", &source_path, &unit_name);
        let template_content: String = utils::read_from_file(&template_file_path).unwrap_or_default();
        let mut modified_content: String = template_content.to_string()
            .replace("{moduleName}", &class_name)
            .replace("{unitName}", &unit_name)
            .replace("{className}", &camel_casename);

        // IsGuard
        let is_guard: bool = self.arg_guard_exist(_cli);
        if is_guard {
            let guard_code: String = self.generate_guard_body(&camel_casename, _cli);
            let guard_header: String = self.generate_guard_header(&camel_casename, _cli);

            modified_content = modified_content.replace("{guardCode}", &guard_code)
                                               .replace("{guardHeader}", &guard_header);
        };

        if fs::write(&template_file_name, modified_content).is_err() {
            print::print_alert("Failed to write modified content to file!");
            return false;
        }
        
        print::print_create("CREATE", 
                            &template_file_name, 
                            &utils::get_size_file(&template_file_name).unwrap_or_default());

        // List Update DPR
        let update: String = format!(
            "  {}.module in 'src\\modules\\{}\\{}.module.pas',",
            &unit_name, &_file_name, &unit_name
        );
        _cli.set_update(update);
        
        true
    }
}

impl CommandModule {
    pub fn new() -> Self {
        Self {}
    }

    pub fn arg_guard_exist(&self, _cli: &mut dyn ICli) -> bool {
        _cli.get_tags().contains_key("--guard") || _cli.get_tags().contains_key("-gu")
    }

    pub fn generate_guard_body(&self, _camel_casename: &str, _cli: &mut dyn ICli) -> String {
        let template_file_path: String = format!("{}/body.pas", _cli.get_path_temp());
        let template_content: String = utils::read_from_file(&template_file_path).unwrap_or_default();
        let modified_content: String = template_content.replace("{className}", &format!("T{}", _camel_casename));

        modified_content
    }

    pub fn generate_guard_header(&self, _camel_casename: &str, _cli: &mut dyn ICli) -> String {
        let template_file_path: String = format!("{}/header.pas", _cli.get_path_temp());
        let template_content: String = utils::read_from_file(&template_file_path).unwrap_or_default();
        let modified_content: String = template_content.replace("{className}", &format!("T{}", _camel_casename));

        modified_content
    }
}
