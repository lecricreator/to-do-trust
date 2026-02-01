use std::{fmt::format, io::{self, BufRead, BufReader, Write}};
use crate::gestionary_file::{self};

pub fn add(argc: usize, args: &Vec<String>) -> io::Result<()>{
    let mut comentary: String = String::new();
    println!("{:?}", args);
    if argc <= 4{
        println!("Need 3 arguments.\n1: action \n2: task\n3. (optinal) commentary");
        return Ok(())
    }else if argc >= 5 {
        comentary = format!("{}",args[3]);
    }
    let mut file = match gestionary_file::find_file(args){
        Ok(f) => f,
        Err(e) => {
            println!("to-do-rustfile not exist.\nTap 'list' for see all the to-do-rustfile.{}", e);
            return Ok(())
        }
    };
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let line:String = line?;
        println!("{}", line)
    }
    let task_done_emoji = '\u{274F}';
    let validate = '\u{2705}';
    //len_total_task = 21
    let task: String = args[3].clone();
    let len_task: usize = task.len();
    let total_space: usize = 19 - len_task;
    let mut space_task: String = String::new();
    for _i in 1..total_space {
        space_task = format!("{} ", space_task);
    }
    let content_file = format!("{}    | {}{}| {}\n", validate, task, space_task, comentary);
    let _total_name_file: String = format!("{}.todoR", task);
    file.write(content_file.as_bytes()).expect("Cannot write in the file : {total_name_file}.");
    Ok(())
}