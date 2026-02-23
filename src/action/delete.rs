use std::{fs};
use crate::manage_file::{self};
use crate::errors::{self};
use std::env;

pub fn delete_action(args: &[String]) -> Result<(), errors::MyError> {
    let file_name: &String = args.first().ok_or_else(|| errors::MyError::ErrArg(errors::ErrArg::ArgNeedFile))?;
    match manage_file::open_file(file_name) {
        Ok(l) => l,
        Err(_e) => {return Err(errors::MyError::ErrFile(errors::ErrFile::ConnotOpenFile))},
    };
    let todotrust_path = env::var("TODORUST_FILE")?;
    let total_file_name = format!("{todotrust_path}/{file_name}.todoR");
    let current_file_name = format!("{todotrust_path}/{file_name}");
    match fs::remove_file(&total_file_name) {
        Ok(_l) => {println!("Delete the {} file.", &total_file_name); return Ok(())},
        Err(_e) => {}
    };
    match fs::remove_file(&current_file_name) {
        Ok(_l) => {println!("Delete the {} file.", &current_file_name); return Ok(())},
        Err(_e) => return  Err(errors::MyError::ErrFile(errors::ErrFile::ConnotRemoveFile))
    };
}