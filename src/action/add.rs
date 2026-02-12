use std::{/*io::{self, BufRead, BufReader, Write}, */fs::File};
use crate::gestionary_file:: replace_file;
//use colored::Colorize;

/*use std::{io::{Write}};
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
} */

pub fn add(argc: usize, args: &Vec<String>){
    replace_file(argc, args, add_in_file, "add".to_string());
    /*
    let mut commentary: String = String::new();
    println!("{:?}", args);
    if !errors::verified_arg(argc, 4) {return Ok(())};
    if argc == 5 {
        commentary = format!("{}",args[4]);
    }
    let mut file = match gestionary_file::find_file(&args[2]){
        Ok(f) => f,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return Ok(())}
    };
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let line:String = line?;
        println!("{}", line);
    }
    let task_done_emoji = '\u{274C}';
    let task: String = args[3].clone();
    let len_task: usize = task.len();
    let total_space: usize = 20 - len_task;
    let mut space_task: String = String::new();
    for _i in 1..total_space {
        space_task = format!("{} ", space_task);
    }
    let content_file = format!("{}   | {}{}| {}\n", task_done_emoji, task.bold().blue(), space_task, commentary.green());
    let _total_name_file: String = format!("{}.todoR", task);
    file.write(content_file.as_bytes()).expect("Cannot write in the file : {total_name_file}.");
    println!("{}", content_file);
    Ok(())*/
}

fn add_in_file (table_line: &Vec<String>, mut file_at_replace: &File, input_index:usize, t: &usize){
    println!("Nothig do now !!!");
}