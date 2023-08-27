use super::clibr_interfaces::ICommand;
use std::rc::Rc;

#[derive(Clone)]
pub struct CommandPair {
    command: Rc<dyn ICommand>,
}

impl CommandPair {
    pub fn new(command: Rc<dyn ICommand>) -> Self {
        Self { command }
    }

    pub fn get_command(&self) -> Rc<dyn ICommand> {
        self.command.clone()
    }
}
