use std::{fmt::{Error, format}, i32, io::{self, BufRead, BufReader, Write}};
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
    let mut index = -1;
    let mut table_line: Vec<String> = vec![];
    for line in reader.lines() {
        let line= line?;
        if index > 1 {
            print!("{} : ", index - 2)
        }else {
            print!("    ");
        }
        println!("{}", line);
        table_line.push(line);
        index += 1;
    }
    index -= 3;
    println!("Choose the index to remove. Ex 1");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input");
    let transf_input_to_int: i32 = match input.trim().parse::<i32>() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Invalid number: {}", e);
            return Ok(());
        }
    };
    if transf_input_to_int > index {
        println!("value out of index of the to-do-rustlist.");
    }
    return Ok(())
}