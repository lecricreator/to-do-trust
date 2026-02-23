use crate::errors::MyError;
use crate::{HEADER_SIZE, errors};

use std::env;
use colored::Colorize;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

pub fn read_file(fd: &mut File) -> Result<String, errors::MyError> {
    let mut content = String::new();
    fd.read_to_string(&mut content)?;
    Ok(content)
}

pub fn open_file(file_name: &String) -> Result<File, errors::MyError> {
    let todotrust_path = env::var("TODORUST_FILE")?;
    let total_file_name = format!("{todotrust_path}/{file_name}.todoR");
    let current_file_name = format!("{todotrust_path}/{file_name}");
    if Path::new(&total_file_name).exists() {
        return Ok(File::open(total_file_name)?)
    }else if Path::new(&current_file_name).exists() {
        return Ok(File::open(current_file_name)?)
    }
    println!("{}", "This argument file doesn't exist".red());
    Err(errors::MyError::ErrFile(errors::ErrFile::FileNotExist))
}

pub fn create_file(name_file: &String) -> Result<File, errors::MyError>{
    let todotrust_path = env::var("TODORUST_FILE")?;
    let total_file_name = format!("{todotrust_path}/{name_file}.todoR");
    let current_file_name = format!("{todotrust_path}/{name_file}");
    if Path::new(&total_file_name).exists() {
        println!("{}{}{}", "No need to create this files. The ".red(), total_file_name.red(), " is already exist.".red());
        return Err(errors::MyError::ErrFile(errors::ErrFile::FileAlreadyExist))
    } else if Path::new(&current_file_name).exists() {
        println!("{}{}{}", "No need to create this files. The ".red(), current_file_name.red(), " is already exist.".red());
        return Err(errors::MyError::ErrFile(errors::ErrFile::FileAlreadyExist))
    }
    let mut file = File::options()
        .write(true)
        .create(true)
        .open(&total_file_name)?;
    println!(
        "Create the file {total_file_name}.\nNow you can add for add goal or show for showing the to-do-trustlist."
    );
    writeln!(
        file,
        "progression: 0/0\n DONE |        TASK        | COMMENTARY        "
    )?;
    writeln!(
        file,
        "------|--------------------|--------------------------------------------------"
    )?;
    Ok(file)
}

pub fn show_and_select_index(file: File, action: &str) -> Result<(usize, Vec<String>),  errors::MyError> {
    let reader = BufReader::new(&file);
    let mut index = 0;
    let mut table_line: Vec<String> = vec![];
    let mut line_string: String;
    let mut nbr_complete = 0;
    for line in reader.lines() {
        line_string = match line {
            Ok(l) => l,
            Err(e) => return Err(MyError::IoError(e)),
        };
        if line_string.starts_with('✅') {
            nbr_complete += 1;
        }
        if index > 2 && index < 10 {
            print!(" {} :", index - HEADER_SIZE)
        } else if index >= 10 {
            print!("{} :", index - HEADER_SIZE)
        } else {
            print!("    ");
        }
        println!("{}", line_string);
        line_string += "\n";
        table_line.push(line_string);
        index += 1;
    }
    println!("Choose the index for {}. Ex 1", action.blue());
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input).err();
    let transf_input_to_int: usize = match input.trim().parse::<usize>() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Invalid number: {}", e);
            return Err(MyError::ErrInput(errors::ErrInput::BadInput));
        }
    };
    if transf_input_to_int + 1 > table_line.len() - HEADER_SIZE {
        return Err(errors::MyError::ErrInput(errors::ErrInput::ValueOutIndex));
    };
    if action == "remove" {
        if table_line[transf_input_to_int + HEADER_SIZE].starts_with("  ✅") && nbr_complete > 0 {
            if nbr_complete > 0 {
                nbr_complete -= 1;
            }
        }
        table_line[0] = format!("progression: {}/{}\n", nbr_complete, table_line.len() - HEADER_SIZE - 1);
    } else if action == "complete task" {
        if table_line[transf_input_to_int + HEADER_SIZE].starts_with("  ❌") {
            nbr_complete += 1;
        } else {
            return Err(errors::MyError::AlreadyComplete)
        }
        table_line[0] = format!("progression: {}/{}\n", nbr_complete, table_line.len() - HEADER_SIZE);
    } else if action == "uncomplete task" {
        if table_line[transf_input_to_int + HEADER_SIZE].starts_with("  ✅") {
            if nbr_complete > 0 {
                nbr_complete -= 1;
            }
        } else {
            return Err(errors::MyError::AlreadyUncomplete)
        }
        table_line[0] = format!("progression: {}/{}\n", nbr_complete, table_line.len() - HEADER_SIZE);
    }
    return Ok((transf_input_to_int, table_line));
}

pub fn replace_file(
    file_name: &String,
    modification: fn(&Vec<String>, &File, usize, &usize) -> Result<(), errors::MyError> ,
    action: &str) -> Result<(), errors::MyError>{
    let file = open_file(file_name)?;
    let (input_index, table_line) = match action {
        "remove" | "complete task" | "uncomplete task" => {
            let res = show_and_select_index(file, action)?;
            res
        },
        "add" => {
            use crate::action::add;
            let Some(table_line) = add::add_task(file, file_name.clone()) else {
                return Err(errors::MyError::ErrInput(errors::ErrInput::BadInput));
            };
            (0, table_line)
        },
        _ => todo!()
    };
    let todotrust_path = env::var("TODORUST_FILE")?;
    let total_file_replace = format!("{todotrust_path}/replace_file");
let file_at_replace: File = File::options()
        .write(true)
        .create(true)
        .open(&total_file_replace)?;
    modify_file(
        &table_line,
        &file_at_replace,
        input_index,
        file_name,
        modification,
        &total_file_replace
    )?;
    Ok(())
}

pub fn modify_file(
    table_line: &Vec<String>,
    file_at_replace: &File,
    input_index: usize,
    file_name: &String,
    f: fn(table_line: &Vec<String>, file_at_replace: &File, input_index: usize, t: &usize) -> Result<(), errors::MyError>, total_file_replace: &String)
    -> Result<(), errors::MyError> {
    for t in 0..table_line.len() {
        f(table_line, &file_at_replace, input_index, &t)?;
    }
    let todotrust_path = env::var("TODORUST_FILE")?;
    let total_file_name = format!("{todotrust_path}/{file_name}.todoR");
    let current_file_name = format!("{todotrust_path}/{file_name}");
    if Path::new(&total_file_name).exists() {
        fs::rename(total_file_replace, total_file_name)?;
        return Ok(());
    }else if Path::new(&current_file_name).exists() {
        fs::rename(total_file_replace, current_file_name)?;
        return Ok(());
    }
    else {
        println!("{total_file_name} / {current_file_name}");
        Err(MyError::ReplaceFile)
    }
}
