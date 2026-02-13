use std::io::{Error, ErrorKind};

mod action;
mod arg;
pub mod errors;
pub mod gestionary_file;

fn main() -> Result<(), std::io::Error>{
    let args: Vec<String> = std::env::args().collect();

    let (_original_path, args) = args.split_first().ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Not sufisaly argument."))?;
    println!("0 is {_original_path}");
    arg::start_program(&args[0..])?;
    Ok(())
}
