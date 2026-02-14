use std::io::{Error, ErrorKind};

mod action;
mod arg;
pub mod errors;
pub mod manage_file;

fn main() -> Result<(), errors::MyError>{
    let args: Vec<String> = std::env::args().collect();

    let (_original_path, args) = args.split_first().ok_or_else(|| errors::MyError::NotSuffisalyArg)?;
    arg::start_program(&args[0..])?;
    Ok(())
}
