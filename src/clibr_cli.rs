use crate::clibr_interfaces::ICli;
use crate::commands::clibr_command_h::CommandHelp;
use crate::commands::clibr_command_v::CommandVersion;
use crate::commands::core::clibr_command_pair::CommandPair;
use crate::commands::core::clibr_typedefs::{ListUpdates, MapCommands, MapOptions, MapTags};
use std::rc::Rc;

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

        options_info.insert(
            "version".to_string(),
            Rc::new(CommandPair::new(command_version.clone())),
        );
        options_info.insert(
            "v".to_string(),
            Rc::new(CommandPair::new(command_version.clone())),
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
        let command_new: Rc<CommandHelp> = Rc::new(CommandHelp::new());

        options_new.insert(
            "application".to_string(),
            Rc::new(CommandPair::new(command_new.clone())),
        );
        options_new.insert(
            "app".to_string(),
            Rc::new(CommandPair::new(command_new.clone())),
        );
        options_new.insert(
            "new".to_string(),
            Rc::new(CommandPair::new(command_new.clone())),
        );
        options_new.insert(
            "--help".to_string(),
            Rc::new(CommandPair::new(command_new.clone())),
        );
        options_new.insert(
            "-h".to_string(),
            Rc::new(CommandPair::new(command_new.clone())),
        );
        self.options_new = Rc::new(options_new.clone());
    }

    // Options Generate
    fn create_options_generate(&mut self, options_generate: &mut MapOptions) {
        let command_generate: Rc<CommandHelp> = Rc::new(CommandHelp::new());

        options_generate.insert(
            "module".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "m".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "controller".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "c".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "service".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "s".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "repository".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "r".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "infra".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "i".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "pipe".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "p".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "all".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "--help".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        options_generate.insert(
            "-h".to_string(),
            Rc::new(CommandPair::new(command_generate.clone())),
        );
        self.options_generate = Rc::new(options_generate.clone());
    }

    // Options internal
    fn create_options_internal(&mut self) {
        let command_internal: Rc<CommandHelp> = Rc::new(CommandHelp::new());

        self.options_internal.insert(
            "--help".to_string(),
            Rc::new(CommandPair::new(command_internal.clone())),
        );
        self.options_internal.insert(
            "-h".to_string(),
            Rc::new(CommandPair::new(command_internal.clone())),
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
    fn path_temp(&self) -> &String {
        &self.path_temp
    }

    fn command_executed(&mut self, value: String) {
        self.command_executed = value;
    }

    fn commands(&self) -> &MapCommands {
        &self.commands
    }

    fn options_internal(&self) -> &MapOptions {
        &self.options_internal
    }

    fn commands_key(&self, key: &String) -> &MapOptions {
        self.commands.get(key).unwrap()
    }

    fn tags(&self) -> &MapTags {
        &self.tags
    }

    fn set_tag_value(&mut self, name: String, value: bool) {
        self.tags.insert(name, value);
    }

    fn updates(&self) -> &ListUpdates {
        &self.updates
    }

    fn set_update(&mut self, value: &String) {
        self.updates.push(value.to_string());
    }
}
