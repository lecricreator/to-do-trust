
use std::{fs::{self, File, OpenOptions}};
use std::io::{BufRead, BufReader,Read, Write, Error};
use std::path::Path;
use colored::Colorize;
use crate::{action::new, errors};

pub fn read_file(fd: &mut File) -> String{
    let mut content: String = String::new();
    fd.read_to_string(&mut content).expect("Error cannot read_to_string in function read_file | file : gestionary");
    content
}

pub fn create_file(name_file: &String){
    let total_name_file: String = format!("{name_file}.todoR");
    if Path::new(&total_name_file).exists() {
        println!("No need to create this files. The {total_name_file} is already exist.");
        return;
    }
    let mut file = File::options()
    .write(true)
    .create(true)
    .open(&total_name_file)
    .expect("Cannot create the {total_name_file}.");
    println!("Create the file {total_name_file}.\nNow you can add for add goal or show for showing the to-do-rustlist.");
    let _ = writeln!(file, "{} / progression: 0/0\nDONE |        TASK        | COMMENTARY        ", &name_file).expect("Cannot write in the file {total_name_file}.");
    _ = writeln!(file,         "-----|--------------------|--------------------------------------------------").expect("Cannot write in the file {total_name_file}.");
}

pub fn find_file(args: &String) -> Result<File, Error>{
    let total_file_name:String = format!("{}.todoR", &args);
        match OpenOptions::new() 
            .read(true)
            .write(true)
            .open(&args) {
                Ok(l) => return Ok(l),
                Err(e) => e,
            };
        match OpenOptions::new() 
            .read(true)
            .write(true)
            .open(total_file_name.clone()) {
                Ok(l) => return Ok(l),
                Err(e) => {return Err(e)},
            };
}

pub fn show_and_select_index(file: File, action: String) -> (i32, Vec<String>){
    let reader = BufReader::new(&file);
    let mut index = 0;
    let mut table_line: Vec<String> = vec![];
    let mut line_string:String;
    for line in reader.lines() {
        match line{
            Ok(l) => line_string = l,
            Err(_) => return (-1, table_line),
        };
        if index > 2 && index < 10 {
            print!(" {} : ", index - 3)
        }else if index >= 10 {
            print!("{} : ", index - 3)
        }else {
            print!("    ");
        }
        println!("{}", line_string);
        line_string += "\n";
        table_line.push(line_string);
        index += 1;
    }
    index -= 4;
    println!("Choose the index for {}. Ex 1", action);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input");
    let transf_input_to_int: i32 = match input.trim().parse::<i32>() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Invalid number: {}", e);
            return (-1, table_line);
        }
    };
    if transf_input_to_int > index 
    {
        println!("value out of index of the to-do-rustlist.");
        return (-1, table_line);
    };
    return (transf_input_to_int, table_line);
}

pub fn replace_file(argc: usize, args: &Vec<String>, modification: fn(&Vec<String>, &File, usize, &usize), action: String) {
    if !errors::verified_arg(argc, 3) {return}
    let file = match find_file(&args[2]){
        Ok(f) => f,
        Err(_e) => {errors::print_error(errors::ErrorName::ErrFileNotFound, args[2].clone()); return}
    };
    let table_line: Vec<String> = Vec::new();
    let mut input_index:usize = 0;
    if action == "remove" || action == "complete task" {
        let (input_index_err, table_line) = show_and_select_index(file, action);
        if input_index_err <= -1 {return;}
        input_index = input_index_err.try_into().unwrap();
    }else {
    }
    let file_at_replace:File = File::options()
    .write(true)
    .create(true)
    .open("replace_file")
    .expect("Cannot create the replace_file.");
    modify_file(&table_line, &file_at_replace, input_index, args, modification);
    return
}

pub fn modify_file(table_line: &Vec<String>, file_at_replace: &File, input_index:usize, args: &Vec<String>, 
    f: fn(table_line: &Vec<String>, file_at_replace: &File, input_index:usize, t: &usize)) {
    for t in 0..table_line.len(){
        f(&table_line, &file_at_replace, input_index, &t);
    }
    let name_old_file: String = format!("{}.todoR", args[2]);
    let _ = fs::rename("replace_file", name_old_file).expect("Cannot rename file. Please contact the dev.");
}