use std::collections::HashMap;

pub type MapOptions<'a> = HashMap<&'a str, &'a CommandPair>;
pub type MapCommands<'a> = HashMap<&'a str, MapOptions<'a>>;
pub type MapTags<'a> = HashMap<&'a str, bool>;
pub type ListUpdates<'a> = Vec<&'a str>;
pub type ListOptions<'a> = Vec<&'a str>;
pub type ListText<'a> = Vec<&'a str>;
pub type ForPair<'a> = (&'a str, &'a CommandPair);