use crate::manage_file::{self, open_file};
use crate::errors;

pub fn show(args: &[String]) -> Result<(), errors::MyError> {
    let Some(file_name) = args.first() else {
        return Err(errors::MyError::FileNotExist)
    };
    let mut fd =  open_file(file_name)?;
    let file_content = manage_file::read_file(&mut fd)?;
    println!("{file_content}");
    Ok(())
}