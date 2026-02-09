use std::{io::{self, BufRead, BufReader, Write}};
use crate::gestionary_file::{self};
use colored::Colorize;
use crate::errors;


pub fn add(argc: usize, args: &Vec<String>) -> io::Result<()>{
    let mut comentary: String = String::new();
    println!("{:?}", args);
    if !errors::verified_arg(argc, 4) {return Ok(())};
    if argc == 5 {
        comentary = format!("{}",args[4]);
    }
    let mut file = match gestionary_file::find_file(&args[2]){
        Ok(f) => f,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return Ok(())}
    };
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let line:String = line?;
        println!("{}", line)
    }
    let task_done_emoji = '\u{274C}';
    //len_total_task = 21
    let task: String = args[3].clone();
    let len_task: usize = task.len();
    let total_space: usize = 20 - len_task;
    let mut space_task: String = String::new();
    for _i in 1..total_space {
        space_task = format!("{} ", space_task);
    }
    let content_file = format!("{}   | {}{}| {}\n", task_done_emoji, task.bold().blue(), space_task, comentary.green());
    let _total_name_file: String = format!("{}.todoR", task);
    file.write(content_file.as_bytes()).expect("Cannot write in the file : {total_name_file}.");
    println!("{}", content_file);
    Ok(())
}