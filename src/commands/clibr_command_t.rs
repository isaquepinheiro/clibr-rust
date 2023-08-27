use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use super::core::clibr_typedefs::ListText;

#[derive(Default)]
pub struct CommandTemplates {}

impl CommandTemplates {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandTemplates {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        let print_text: ListText = vec![
            "".to_string(),
            "|----------------------|".to_string(),
            "| CLIBr - Templates    |".to_string(),
            "|----------------------|-------------------------------------------|".to_string(),
            "|        Name          |               Descripton                  |".to_string(),
            "|----------------------|-------------------------------------------|".to_string(),
            "| handler.pas          | Default Route Handler                     |".to_string(),
            "| module.app.pas       | Default Application Module                |".to_string(),
            "| module.pas           | Default Modules                           |".to_string(),
            "| controller.pas       | Default Controller                        |".to_string(),
            "| service.pas          | Default Service                           |".to_string(),
            "| vcl.project.pas      | Default VCL Server Project                |".to_string(),
            "| vcl.project.unit.pas | Default VCL Server Unit                   |".to_string(),
            "| vcl.project.form.pas | Default VCL Server Form                   |".to_string(),
            "| console.projetc.pas  | Default Console Project                   |".to_string(),
            "| horse.project.pas    | Horse Pattern Projec                      |".to_string(),
            "| horse.handler.pas    | Horse Pattern Route Handler               |".to_string(),
            "| body.pas             | Guad Routes Body                          |".to_string(),
            "| header.pas           | Guad Routes Header                        |".to_string(),
            "| pipe.pas             | Pipe (Transformation/Validation)          |".to_string(),
            "|----------------------|-------------------------------------------|".to_string(),
            "".to_string(),
        ];        

        print::print_help(&print_text.join("\n"));
        true
    }
}
