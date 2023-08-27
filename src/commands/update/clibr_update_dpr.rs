use std::fs;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Write};
use std::{fs::File, path::Path};
use crate::commands::core::clibr_print::print;
use crate::commands::core::clibr_utils::utils;
use super::super::core::clibr_interfaces::{ICli, ICommand};


#[derive(Default)]
pub struct CommandUpdateDpr {}

impl CommandUpdateDpr {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for CommandUpdateDpr {
    fn execute(&self, _dir_name: &str, _file_name: &str, _cli: &mut dyn ICli) -> bool {
        let current_dir: PathBuf = Path::new(".").join(_dir_name);
        let current_dir_clone: PathBuf = current_dir.clone();
        let current_dir: PathBuf = fs::canonicalize(current_dir).unwrap_or(current_dir_clone);

        let mut dpr_file_name: String = String::new();
        
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "dpr" {
                            let dpr_file_path: String = entry.path().parent().unwrap().to_string_lossy().into_owned();
                            dpr_file_name = dpr_file_path.clone() + "\\" + entry.path().file_name().unwrap().to_str().unwrap();
                            break;
                        }
                    }
                }
            }
        }
        
        if dpr_file_name.is_empty() {
            print::print_alert("Error: DPR file not found.");
            return false;
        }

        let mut lines: Vec<String> = Vec::new();
        if let Ok(file) = File::open(&dpr_file_name) {
            let reader: BufReader<File> = BufReader::new(file);
            for line in reader.lines().flatten() {
                lines.push(line);
            }
        }

        // Find uses clause
        let mut uses_index: Option<usize> = None;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("uses") {
                uses_index = Some(i);
                break;
            }
        }

        let uses_index: usize = match uses_index {
            Some(index) => index,
            None => {
                print::print_alert("Error: 'uses' clause not found in the DPR file.");
                return false;
            }
        };

        // Find ";" after uses clause
        let mut semicolon_index: Option<usize> = None;
        for (i, line) in lines.iter().enumerate().skip(uses_index + 1) {
            if line.contains(';') {
                semicolon_index = Some(i);
                break;
            }
        }

        let mut semicolon_index: usize = match semicolon_index {
            Some(index) => index,
            None => {
                print::print_alert("Error: Semicolon not found after the 'uses' clause in the DPR file.");
                return false;
            }
        };

        // Replace ";" to ","
        lines[semicolon_index] = lines[semicolon_index].replace(';', ",");

        let updates: &Vec<String> = _cli.get_updates();
        for unit_name in updates {
            // The line does not exist.
            if !lines.iter().any(|line| equals_ignore_case(line, unit_name)) {
                semicolon_index += 1;
                lines.insert(semicolon_index, unit_name.to_string());
            }
        }

        // Replace "," to ";"    
        lines[semicolon_index] = lines[semicolon_index].replace(',', ";");

        if let Ok(mut file) = File::create(&dpr_file_name) {
            for updated_line in &lines {
                writeln!(file, "{}", updated_line).unwrap();
            }
        }

        let update: String = format!("{}/{}", _dir_name, dpr_file_name);
        print::print_update("UPDATE", 
                             &update, 
                          &utils::get_size_file(&dpr_file_name).unwrap_or_default());

        true
    }
}

fn equals_ignore_case(str1: &str, str2: &str) -> bool {
    str1.len() == str2.len() && str1.chars()
                                    .zip(str2.chars())
                                    .all(|(c1, c2)| c1.to_lowercase().eq(c2.to_lowercase()))
}