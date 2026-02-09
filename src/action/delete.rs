use std::{fs};
use crate::gestionary_file::{self};
use crate::errors::{self};

pub fn delete(argc: usize, args: &Vec<String>) {
    if !errors::verified_arg(argc, 3) {return}
    println!("TEST");
    match gestionary_file::find_file(&args[2]) {
        Ok(l) => l,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return},
    };
    println!("Failed to delete file1");
    let total_name: String = format!("{}.todoR", args[2]);
    match fs::remove_file(total_name) {
        Ok(_l) => return,
        Err(e) => e,
    };
    println!("Failed to delete file2");
    match fs::remove_file(&args[2]) {
        Ok(_l) => return,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return}
    }
}