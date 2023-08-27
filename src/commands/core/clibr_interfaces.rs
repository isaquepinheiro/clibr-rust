use super::clibr_typedefs::{ListUpdates, MapCommands, MapOptions, MapTags};

/// Trait representing a command-line interface (CLI).
pub trait ICli {
    /// Gets the path to the temporary directory.
    fn get_path_temp(&self) -> &str;

    /// Gets a reference to the map of commands.
    fn get_commands(&self) -> &MapCommands;

    /// Gets a reference to the map of options for a specific command key.
    fn get_commands_key(&self, key: &str) -> &MapOptions;

    /// Gets a reference to the map of internal options.
    fn get_options_internal(&self) -> &MapOptions;

    /// Gets a reference to the map of tags.
    fn get_tags(&self) -> &MapTags;

    /// Gets a reference to the list of updates.
    fn get_updates(&self) -> &ListUpdates;

    /// Gets the executed command.
    fn get_command_executed(&self) -> &str;

    /// Sets the executed command.
    fn set_command_executed(&mut self, value: String);

    /// Sets the value of a specific tag.
    fn set_tag_value(&mut self, name: String, value: bool);

    /// Sets the update value.
    fn set_update(&mut self, value: String);
}

/// Trait representing a command.
pub trait ICommand {
    /// Executes the command.
    ///
    /// # Arguments
    ///
    /// * `dir_name` - The directory name.
    /// * `file_name` - The file name.
    /// * `cli` - The command-line interface.
    ///
    /// # Returns
    ///
    /// Returns `true` if the command was executed successfully, otherwise `false`.
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool;
}
