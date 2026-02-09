use std::{io::{Write}};
use std::fs::{File};
use crate::gestionary_file::{self};
use crate::errors::{self};

pub fn remove(argc: usize, args: &Vec<String>){
    if !errors::verified_arg(argc, 3) {return}
    let file = match gestionary_file::find_file(&args[2]){
        Ok(f) => f,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return}
    };
    let (input_index_err, table_line) = gestionary_file::show_and_select_index(file, "remove".to_string());
    if input_index_err <= -1 {return;}
    let input_index:usize = input_index_err.try_into().unwrap();
    let file_at_replace:File = File::options()
    .write(true)
    .create(true)
    .open("replace_file")
    .expect("Cannot create the replace_file.");
    gestionary_file::modify_file(&table_line, &file_at_replace, input_index, args, remove_line);
    return
}

fn remove_line(table_line: &Vec<String>, mut file_at_replace: &File, input_index:usize, t: &usize) {
        if *t != input_index + 3 {
            file_at_replace.write(table_line[*t].as_bytes()).expect("Can not write in file");
        }
}