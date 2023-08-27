use super::super::clibr_command_all::CommandAll;
use super::super::clibr_command_c::CommandController;
use super::super::clibr_command_dpr::CommandGenerateProject;
use super::super::clibr_command_h::CommandHelp;
use super::super::clibr_command_handler::CommandRouteHandler;
use super::super::clibr_command_i::CommandInfra;
use super::super::clibr_command_info::CommandInfo;
use super::super::clibr_command_m::CommandModule;
use super::super::clibr_command_pipe::CommandTransformPipe;
use super::super::clibr_command_r::CommandRepository;
use super::super::clibr_command_s::CommandService;
use super::super::clibr_command_t::CommandTemplates;
use super::super::clibr_command_v::CommandVersion;
use super::super::horse::clibr_command_dpr_horse::CommandGenerateProjectHorse;
use super::super::horse::clibr_command_handler_horse::CommandRouteHandlerHorse;
use super::super::vcl::clibr_command_dpr_vcl::CommandGenerateProjectVCL;
use super::clibr_command_pair::CommandPair;
use super::clibr_interfaces::ICli;
use super::clibr_typedefs::{ListUpdates, MapCommands, MapOptions, MapTags};
use std::rc::Rc;

#[derive(Clone)]
pub struct Cli {
    path_temp: String,
    command_executed: String,
    options_info: Rc<MapOptions>,
    options_help: Rc<MapOptions>,
    options_new: Rc<MapOptions>,
    options_generate: Rc<MapOptions>,
    options_internal: MapOptions,
    commands: MapCommands,
    tags: MapTags,
    updates: ListUpdates,
}

impl Cli {
    /// Creates a new instance of the Cli struct.
    pub fn new(path_temp: String) -> Self {
        let mut options_info: MapOptions = MapOptions::new();
        let mut options_help: MapOptions = MapOptions::new();
        let mut options_new: MapOptions = MapOptions::new();
        let mut options_generate: MapOptions = MapOptions::new();

        let mut cli = Self {
            path_temp: path_temp.to_string(),
            command_executed: String::new(),
            options_new: Rc::new(MapOptions::new()),
            options_info: Rc::new(MapOptions::new()),
            options_help: Rc::new(MapOptions::new()),
            options_generate: Rc::new(MapOptions::new()),
            options_internal: MapOptions::new(),
            commands: MapCommands::new(),
            tags: MapTags::new(),
            updates: ListUpdates::new(),
        };
        cli.create_options_info(&mut options_info);
        cli.create_options_help(&mut options_help);
        cli.create_options_new(&mut options_new);
        cli.create_options_generate(&mut options_generate);
        cli.create_options_internal();
        cli.create_tags();
        cli.create_commands();
        cli
    }

    /// Options Info
    fn create_options_info(&mut self, options_info: &mut MapOptions) {
        let command_version: Rc<CommandVersion> = Rc::new(CommandVersion::new());
        let command_info: Rc<CommandInfo> = Rc::new(CommandInfo::new());
        let command_templates: Rc<CommandTemplates> = Rc::new(CommandTemplates::new());

        options_info.insert(
            "version".to_string(),
            Rc::new(CommandPair::new(command_version.clone())),
        );
        options_info.insert(
            "v".to_string(),
            Rc::new(CommandPair::new(command_version.clone())),
        );
        options_info.insert(
            "info".to_string(),
            Rc::new(CommandPair::new(command_info.clone())),
        );
        options_info.insert(
            "i".to_string(),
            Rc::new(CommandPair::new(command_info.clone())),
        );
        options_info.insert(
            "templates".to_string(),
            Rc::new(CommandPair::new(command_templates.clone())),
        );
        options_info.insert(
            "t".to_string(),
            Rc::new(CommandPair::new(command_templates.clone())),
        );
        self.options_info = Rc::new(options_info.clone());
    }

    // Options Help
    fn create_options_help(&mut self, options_help: &mut MapOptions) {
        let command_help: Rc<CommandHelp> = Rc::new(CommandHelp::new());

        options_help.insert(
            "--help".to_string(),
            Rc::new(CommandPair::new(command_help.clone())),
        );
        options_help.insert(
            "-h".to_string(),
            Rc::new(CommandPair::new(command_help.clone())),
        );
        self.options_help = Rc::new(options_help.clone());
    }

    // Options New
    fn create_options_new(&mut self, options_new: &mut MapOptions) {
        let command_app: Rc<CommandGenerateProject> = Rc::new(CommandGenerateProject::new());
        let command_help: Rc<CommandHelp> = Rc::new(CommandHelp::new());

        options_new.insert(
            "application".to_string(),
            Rc::new(CommandPair::new(command_app.clone())),
        );
        options_new.insert(
            "app".to_string(),
            Rc::new(CommandPair::new(command_app.clone())),
        );
        options_new.insert(
            "new".to_string(),
            Rc::new(CommandPair::new(command_app.clone())),
        );
        options_new.insert(
            "--help".to_string(),
            Rc::new(CommandPair::new(command_help.clone())),
        );
        options_new.insert(
            "-h".to_string(),
            Rc::new(CommandPair::new(command_help.clone())),
        );
        self.options_new = Rc::new(options_new.clone());
    }

    // Options Generate
    fn create_options_generate(&mut self, options_generate: &mut MapOptions) {
        let command_m: Rc<CommandModule> = Rc::new(CommandModule::new());
        let command_c: Rc<CommandController> = Rc::new(CommandController::new());
        let command_s: Rc<CommandService> = Rc::new(CommandService::new());
        let command_r: Rc<CommandRepository> = Rc::new(CommandRepository::new());
        let command_infra: Rc<CommandInfra> = Rc::new(CommandInfra::new());
        let command_pipe: Rc<CommandTransformPipe> = Rc::new(CommandTransformPipe::new());
        let command_help: Rc<CommandHelp> = Rc::new(CommandHelp::new());

        options_generate.insert(
            "module".to_string(),
            Rc::new(CommandPair::new(command_m.clone())),
        );
        options_generate.insert(
            "m".to_string(),
            Rc::new(CommandPair::new(command_m.clone())),
        );
        options_generate.insert(
            "controller".to_string(),
            Rc::new(CommandPair::new(command_c.clone())),
        );
        options_generate.insert(
            "c".to_string(),
            Rc::new(CommandPair::new(command_c.clone())),
        );
        options_generate.insert(
            "service".to_string(),
            Rc::new(CommandPair::new(command_s.clone())),
        );
        options_generate.insert(
            "s".to_string(),
            Rc::new(CommandPair::new(command_s.clone())),
        );
        options_generate.insert(
            "repository".to_string(),
            Rc::new(CommandPair::new(command_r.clone())),
        );
        options_generate.insert(
            "r".to_string(),
            Rc::new(CommandPair::new(command_r.clone())),
        );
        options_generate.insert(
            "infra".to_string(),
            Rc::new(CommandPair::new(command_infra.clone())),
        );
        options_generate.insert(
            "i".to_string(),
            Rc::new(CommandPair::new(command_infra.clone())),
        );
        options_generate.insert(
            "pipe".to_string(),
            Rc::new(CommandPair::new(command_pipe.clone())),
        );
        options_generate.insert(
            "p".to_string(),
            Rc::new(CommandPair::new(command_pipe.clone())),
        );
        options_generate.insert(
            "all".to_string(),
            Rc::new(CommandPair::new(Rc::new(CommandAll::new()))),
        );
        options_generate.insert(
            "--help".to_string(),
            Rc::new(CommandPair::new(command_help.clone())),
        );
        options_generate.insert(
            "-h".to_string(),
            Rc::new(CommandPair::new(command_help.clone())),
        );
        self.options_generate = Rc::new(options_generate.clone());
    }

    // Options internal
    fn create_options_internal(&mut self) {
        self.options_internal.insert(
            "handler".to_string(),
            Rc::new(CommandPair::new(Rc::new(CommandRouteHandler::new()))),
        );
        self.options_internal.insert(
            "horse-app".to_string(),
            Rc::new(CommandPair::new(Rc::new(CommandGenerateProjectHorse::new()))),
        );
        self.options_internal.insert(
            "horse-handler".to_string(),
            Rc::new(CommandPair::new(Rc::new(CommandRouteHandlerHorse::new()))),
        );
        self.options_internal.insert(
            "vcl-app".to_string(),
            Rc::new(CommandPair::new(Rc::new(CommandGenerateProjectVCL::new()))),
        );
    }

    // Commands
    fn create_commands(&mut self) {
        let options_info: Rc<MapOptions> = self.options_info.clone();
        let options_help: Rc<MapOptions> = self.options_help.clone();
        let options_new: Rc<MapOptions> = self.options_new.clone();
        let options_generate: Rc<MapOptions> = self.options_generate.clone();

        // New
        self.commands.insert("new".to_string(), options_new.clone());
        self.commands.insert("n".to_string(), options_new.clone());
        // Generate
        self.commands
            .insert("generate".to_string(), options_generate.clone());
        self.commands
            .insert("g".to_string(), options_generate.clone());
        // Info
        self.commands
            .insert("info".to_string(), options_info.clone());
        self.commands.insert("i".to_string(), options_info.clone());
        // Templates
        self.commands
            .insert("templates".to_string(), options_info.clone());
        self.commands.insert("t".to_string(), options_info.clone());
        // Version
        self.commands
            .insert("version".to_string(), options_info.clone());
        self.commands.insert("v".to_string(), options_info.clone());
        // Help
        self.commands
            .insert("--help".to_string(), options_help.clone());
        self.commands.insert("-h".to_string(), options_help.clone());
    }

    // Tags
    fn create_tags(&mut self) {
        self.tags.insert("-gu".to_string(), false);
        self.tags.insert("--guard".to_string(), false);
        self.tags.insert("--horse".to_string(), false);
        self.tags.insert("--vcl".to_string(), false);
    }
}

impl ICli for Cli {
    fn get_path_temp(&self) -> &str {
        &self.path_temp
    }

    fn get_command_executed(&self) -> &str {
        &self.command_executed
    }

    fn get_commands(&self) -> &MapCommands {
        &self.commands
    }

    fn get_options_internal(&self) -> &MapOptions {
        &self.options_internal
    }

    fn get_commands_key(&self, key: &str) -> &MapOptions {
        self.commands.get(key).unwrap()
    }

    fn get_tags(&self) -> &MapTags {
        &self.tags
    }

    fn get_updates(&self) -> &ListUpdates {
        &self.updates
    }

    fn set_tag_value(&mut self, name: String, value: bool) {
        self.tags.insert(name, value);
    }

    fn set_command_executed(&mut self, value: String) {
        self.command_executed = value;
    }

    fn set_update(&mut self, value: String) {
        self.updates.push(value.to_string());
    }
}
