use std::{fs};
use crate::manage_file::{self};
use crate::errors::{self};

pub fn delete_action(args: &[String]) -> Result<(), errors::MyError> {
    let file_name: &String = args.first().ok_or_else(|| errors::MyError::ErrArg(errors::ErrArg::ArgNeedFile))?;
    match manage_file::open_file(file_name) {
        Ok(l) => l,
        Err(_e) => {return Err(errors::MyError::ErrFile(errors::ErrFile::ConnotOpenFile))},
    };
    let total_name: String = format!("{file_name}.todoR");
    match fs::remove_file(total_name) {
        Ok(_l) => return Ok(()),
        Err(_e) => {}
    };
    match fs::remove_file(file_name) {
        Ok(_l) => return Ok(()),
        Err(_e) => return  Err(errors::MyError::ErrFile(errors::ErrFile::ConnotRemoveFile))
    }
}