use super::core::clibr_interfaces::{ICli, ICommand};
use super::core::clibr_print::print;
use super::core::clibr_typedefs::ListText;

#[derive(Default)]
pub struct CommandHelp {}

impl CommandHelp {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandHelp {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        let command_executed: &str = _cli.get_command_executed();

        if command_executed == "generate" || command_executed == "g" {
            self.execute_internal_g();
        } else if command_executed == "new" || command_executed == "n" {
            self.execute_internal_n();
        } else {
            self.execute_internal();
        }
        true
    }
}

impl CommandHelp {
    // Help for clibr command
    fn execute_internal(&self) {
        let print_text: ListText = vec![
            "".to_string(),
            "CLIBr".to_string(),
            "Usage: clibr <command> [argument] [filename] [tag]".to_string(),
            "\x1B[36mcommands:\x1B[0m".to_string(),
            "  n, new           create new delphi project (console default)".to_string(),
            "  g, generate      create new argument".to_string(),
            "  v, version       show version".to_string(),
            "  i, info          show informations".to_string(),
            "  t, templates     show templates".to_string(),
            "\x1B[31marguments:\x1B[0m".to_string(),
            "  app, application create new delphi project".to_string(),
            "  handler          create route handle".to_string(),
            "  module           create module".to_string(),
            "  controller       create controller".to_string(),
            "  service          create service".to_string(),
            "  all              create all (handler, module, controller and service)".to_string(),
            "\x1B[32mtags:\x1B[0m".to_string(),
            "  --vcl            create new project VCL pattern".to_string(),
            "  --horse          create new project Horse pattern (console)".to_string(),
            "  -gu, --guard     add security guards".to_string(),
            "  -h, --help       show help".to_string(),
            "\x1B[33mFor more information, please refer to the documentation.\x1B[0m".to_string(),
            "\x1B[34mhttps://dmfbr-en.docs-br.com\x1B[0m".to_string(),
            "".to_string(),
        ];

        print::print_help(&print_text.join("\n"));
    }

    // Help for generate command
    fn execute_internal_g(&self) {
        let print_text: ListText = vec![
            "".to_string(),
            "CLIBr".to_string(),
            "Usage: clibr g <argument> <filename> [tag]".to_string(),
            "\x1B[35mcommands:\x1B[0m".to_string(),
            "  g, generate      generates the desired argument".to_string(),
            "\x1B[31marguments:\x1B[0m".to_string(),
            "  handler          create route handle".to_string(),
            "  module           create module".to_string(),
            "  controller       create controller".to_string(),
            "  service          create service".to_string(),
            "  all              create all (handler, module, controller and service)".to_string(),
            "\x1B[32mtags:\x1B[0m".to_string(),
            "  --horse          create Horse pattern (console)".to_string(),
            "  -gu, --guard     adds security guards".to_string(),
            "  -h, --help       show help".to_string(),
            "\x1B[36msamples:\x1B[0m".to_string(),
            "  clibr g handler products".to_string(),
            "  clibr g module products --horse --guard".to_string(),
            "  clibr g controller products".to_string(),
            "  clibr g service products".to_string(),
            "  clibr g all products".to_string(),
            "\x1B[33mFor more information, please refer to the documentation.\x1B[0m".to_string(),
            "\x1B[34mhttps://dmfbr-en.docs-br.com\x1B[0m".to_string(),
            "".to_string(),
        ];

        print::print_help(&print_text.join("\n"));
    }

    // Help for new command
    fn execute_internal_n(&self) {
        let print_text: ListText = vec![
            "".to_string(),
            "CLIBr".to_string(),
            "Usage: clibr n <argument> <filename> [tag]".to_string(),
            "\x1B[32mcommands:\x1B[0m".to_string(),
            "  n, new           create new delphi project (console default)".to_string(),
            "\x1B[31marguments:\x1B[0m".to_string(),
            "  app, application".to_string(),
            "\x1B[35mtags:\x1B[0m".to_string(),
            "  --horse          create Horse pattern".to_string(),
            "  -gu, --guard     adds security guards".to_string(),
            "  -h, --help       show help".to_string(),
            "\x1B[36msamples:\x1B[0m".to_string(),
            "  clibr n app app_ping".to_string(),
            "  clibr n app app_ping --horse".to_string(),
            "  clibr n app app_ping --guard".to_string(),
            "\x1B[33mFor more information, please refer to the documentation.\x1B[0m".to_string(),
            "\x1B[34mhttps://dmfbr-en.docs-br.com\x1B[0m".to_string(),
            "".to_string(),
        ];

        print::print_help(&print_text.join("\n"));
    }
}
