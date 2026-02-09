use std::{io::{Write}};
use std::fs::{File};
use crate::gestionary_file::{self};
use crate::errors::{self};

pub fn uncomplete(argc: usize, args: &Vec<String>){
    if !errors::verified_arg(argc, 3) {return};
    let file = match gestionary_file::find_file(&args[2]){
        Ok(f) => f,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return}
    };
    let (input_index_err, table_line) = gestionary_file::show_and_select_index(file, "uncomplete task".to_string());
    if input_index_err <= -1 {return;}
    let input_index:usize = input_index_err.try_into().unwrap();
    let file_at_replace:File = File::options()
    .write(true)
    .create(true)
    .open("replace_file")
    .expect("Cannot create the replace_file.");
    gestionary_file::modify_file(&table_line, &file_at_replace, input_index, args, uncomplete_file);
    return
}

fn uncomplete_file(table_line: &Vec<String>, mut file_at_replace: &File, input_index:usize, t: &usize){
    if *t == input_index + 3 {
        let modify_str = table_line[*t].replace("\u{2705}", "\u{274C}");
        file_at_replace.write(modify_str.as_bytes()).expect("Can not write the modification in file.");
    }else{
        file_at_replace.write(table_line[*t].as_bytes()).expect("Can not write in file.");
    }

}