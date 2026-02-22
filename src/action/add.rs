use crate::errors;
use crate::manage_file::replace_file;
use colored::Colorize;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn add_action(args: &[String]) -> Result<(), errors::MyError> {
    let file_name: &String = args.first().ok_or_else(|| errors::MyError::ErrArg(errors::ErrArg::ArgNeedFile))?;
    replace_file(file_name, add_in_file, "add")?;
    Ok(())
}

fn add_in_file(
    table_line: &Vec<String>,
    mut file_at_replace: &File,
    _input_index: usize,
    t: &usize,
) -> Result<(), errors::MyError>{
    let line = format!("{}", table_line[*t]);
    file_at_replace.write(line.as_bytes())?;
    Ok(())
}

pub fn add_task(file: File, _name_file: String) -> Option<Vec<String>> {
    let reader = BufReader::new(&file);
    let mut nbr_complete: u8 = 0;
    let mut table_line = reader
        .lines()
        .filter_map(|line| {
            let mut line_string = line.ok()?;
            if line_string.starts_with('✅') {
                nbr_complete += 1;
            }
            println!("{}", line_string);
            line_string += "\n";
            Some(line_string)
        })
        .collect::<Vec<String>>();
    println!("\nwrite the task for add in the to-do-RList. Ex task1");
    let mut input_task = String::new();
    let len_input_task = 
    loop {
        if let Ok(len) = std::io::stdin().read_line(&mut input_task) {
            break len;
        }
        println!("Invalid value, try again");
    };
    if input_task.trim().is_empty() {
        println!("Input task is empty.");
        return None
    }
        println!(
        "write the commentary for add in the to-do-RList. It's not obligatory. Ex commentary1"
    );
    let mut input_commentary: String = String::new();
    std::io::stdin()
        .read_line(&mut input_commentary).err();

    if len_input_task >= 20 {
        input_task = format!("{}.", &input_task[..17]);
    }
    let content_file = format!(
        "❌   | {:^19}| {}\n",
        input_task.trim().bold().blue(),
        input_commentary.trim().green()
    );
    table_line.push(content_file);
    table_line[0] = format!(
        "progression: {}/{}\n",
        nbr_complete,
        table_line.len() - 3
    );
    table_line.iter().for_each(|l| print!("{l}"));
    Some(table_line)
}
