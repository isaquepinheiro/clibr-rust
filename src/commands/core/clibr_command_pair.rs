use crate::interfaces::ICommand;
use std::rc::Rc;

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

    pub fn set_command(&mut self, command: Rc<dyn ICommand>) {
        self.command = command;
    }
}