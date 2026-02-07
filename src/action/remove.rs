use std::{fs, io::{Write}};
use std::fs::{File};
use crate::gestionary_file::{self};

pub fn remove(argc: usize, args: &Vec<String>){
    if argc != 3{
        println!("Need 3 arguments.\n1: action \n2: task\n3. (optinal) commentary");
    }
    let file = match gestionary_file::find_file(args){
        Ok(f) => f,
        Err(e) => {println!("to-do-rustfile not exist.\nTap 'list' for see all the to-do-rustfile.{}", e); return}
    };
    let input_index_err:i32;
    let table_line:Vec<String>;
    (input_index_err, table_line) = gestionary_file::show_and_select_index(file, "remove".to_string());
    if input_index_err == -1 {return;}
    let input_index:usize = input_index_err.try_into().unwrap();
    let mut file_at_replace:File = File::options()
    .write(true)
    .create(true)
    .open("replace_file")
    .expect("Cannot create the replace_file.");
    for t in 0..table_line.len(){
        if t != input_index + 3 {
            file_at_replace.write(table_line[t].as_bytes()).expect("Can not write in file");
        }
    }
    let name_old_file: String = format!("{}.todoR", args[2]);
    let _ = fs::rename("replace_file", name_old_file).expect("Cannot rename file. Please contact the dev.");
    return
}