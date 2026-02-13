use std::io::{Error, ErrorKind};

use crate::gestionary_file::{self, open_file};
//use crate::errors;

pub fn show(args: &[String]) -> Result<(), Error> {
    let Some(file_name) = args.first() else {
        return Err(Error::new(ErrorKind::InvalidInput, "Not sufisaly argument, need fileName."))
    };
    let mut fd =  open_file(file_name)?;
    let file_content = gestionary_file::read_file(&mut fd)?;
    println!("{file_content}");
    Ok(())
}