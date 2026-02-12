use std::{io::{Write}};
use std::fs::{File};
use crate::gestionary_file;
use crate::errors;

pub fn remove(argc: usize, args: &Vec<String>){
    gestionary_file::replace_file(argc, args, remove_line, "remove".to_string());}

fn remove_line(table_line: &Vec<String>, mut file_at_replace: &File, input_index:usize, t: &usize) {
        if *t != input_index + 3 {
            file_at_replace.write(table_line[*t].as_bytes()).expect("Can not write in file");
        }
}