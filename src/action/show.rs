use crate::gestionary_file;
use crate::errors::{self};

pub fn show(argc: usize, args: &Vec<String>) {
    if !errors::verified_arg(argc, 3) {return};
    let mut file = match gestionary_file::find_file(&args[2]){
        Ok(f) => f,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return}
    };
    let file_content = gestionary_file::read_file(&mut file);
    println!("{file_content}");
}