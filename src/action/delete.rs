use std::{fmt::format, fs, io::Write};
use crate::gestionary_file::{self};
use crate::errors::{self};

pub fn delete(argc: usize, args: &Vec<String>) {
    if !errors::verified_arg(argc, 3) {return}
    match gestionary_file::find_file(&args[2]) {
        Ok(l) => l,
        Err(_e) => return,
    };
    let total_name: String = format!("{}.todoR", args[2]);
    match fs::remove_file(total_name) {
        Ok(_l) => return,
        Err(_e) => (),
    }
    match fs::remove_file(&args[2]) {
        Ok(_l) => return,
        Err(_e) => {errors::print_error(errors::ErrorName::err_file_not_found, args[2].clone()); return}
    }
}