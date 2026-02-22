mod action;
mod arg;
pub mod errors;
pub mod manage_file;

const HEADER_SIZE: usize = 3;

fn main() -> Result<(), errors::MyError>{
    let args: Vec<String> = std::env::args().collect();

    let (_original_path, args) = args.split_first().ok_or_else(|| errors::MyError::ErrArg(errors::ErrArg::NoArgContactMe))?;
    if let Err(e) = arg::start_program(&args[0..]) {
        println!("{}", e)
    }
    Ok(())
}
