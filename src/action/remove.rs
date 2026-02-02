use std::{fmt::format, io::{self, BufRead, BufReader, Write}};
use crate::gestionary_file::{self};

pub fn remove(argc: usize, args: &Vec<String>) -> io::Result<()>{
    if argc != 3{
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
        let line: String = line?;
        //if line != &args[2];
        println!("{}", line);
    }

    return Ok(())
}