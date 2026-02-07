use std::io::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader,Read, Write};
use std::path::Path;

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
    let _ = writeln!(file, "{}\nDONE |        TASK        | COMMENTARY        ", &name_file).expect("Cannot write in the file {total_name_file}.");
    _ = writeln!(file,         "-----|--------------------|--------------------------------------------------").expect("Cannot write in the file {total_name_file}.");
}

pub fn find_file(args: &Vec<String>) -> Result<File, Error>{
    let total_file_name:String = format!("{}.todoR", &args[2]);
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(&&args[2])
        .or_else(|_| {
            OpenOptions::new()
            .read(true)
            .write(true)
            .open(&total_file_name)
        })
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
        if index > 2 {
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