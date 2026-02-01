use std::io::{self, Write, BufRead, BufReader};
use crate::gestionary_file::{self};

pub fn add(argc: usize, args: &Vec<String>) -> io::Result<()>{
    if argc <= 3{
        println!("Need 3 arguments.\n1: action \n2: task\n3. (optinal) commentary");
        return Ok(())
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
    let content_file = format!("{}    | {}\n", validate, &args[3]);
    let _total_name_file: String = format!("{}.todoR", &args[2]);
    file.write(content_file.as_bytes()).expect("Cannot write in the file : {total_name_file}.");
    Ok(())
}