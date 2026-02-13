use std::io::{Error, ErrorKind};

use crate::gestionary_file;

pub fn new_action(args: &[String]) -> Result<(), std::io::Error> {
    println!("LU");
    let Some(file_name) = args.first() else {
        Err(Error::new(ErrorKind::InvalidInput, "Not sufisaly argument, need filename."))?
    };
    gestionary_file::create_file(file_name);
    Ok(())
}