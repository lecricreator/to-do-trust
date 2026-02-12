use std::fs;
use crate::errors;

use crate::errors::ErrorName;

pub fn list(){
    let entries = match fs::read_dir(".") {
        Ok(l) => l, 
        Err(e) => {errors::print_error(ErrorName::ErrReadDirectory, e.to_string()); return}};
    let mut table_str: Vec<String> = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            if let Some(name) = file_name.to_str() {
                if name.ends_with(".todoR") {
                    table_str.push(name.to_string());
                }
            }
        }
    }
    println!("You have {} todoR.", table_str.len());
    for i in table_str {
        println!("{i}");
    }
}