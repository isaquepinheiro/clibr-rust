use super::clibr_command_pair::CommandPair;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

pub type MapOptions = HashMap<String, Rc<CommandPair>>;
pub type MapCommands = HashMap<String, Rc<MapOptions>>;
pub type MapTags = HashMap<String, bool>;
pub type ListUpdates = Vec<String>;
pub type ListOptions = Vec<String>;
pub type ListText = Vec<String>;
//pub type ForPair = (String, CommandPair);
