use crate::errors;
use crate::manage_file::replace_file;
use colored::Colorize;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn add(args: &[String]) -> Result<(), errors::MyError> {
    let file_name: &String = args.first().ok_or_else(|| errors::MyError::ActionNeeded)?;
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
    file_at_replace
        .write(line.as_bytes())?;
    Ok(())
}

pub fn add_task(file: File, name_file: String) -> Option<Vec<String>> {
    let reader = BufReader::new(&file);
    let mut nbr_complete: u8 = 0;
    let mut table_line = reader
        .lines()
        .filter_map(|line| {
            let mut line_string = line.ok()?;

            println!("{line_string}");
            if line_string.starts_with('✅') {
                nbr_complete += 1;
            }
            line_string += "\n";

            Some(line_string)
        })
        .collect::<Vec<String>>();

    println!("\nwrite the task for add in the to-do-RList. Ex task1");
    let mut input_task = String::new();
    let mut len_input_task = loop {
        if let Ok(len) = std::io::stdin().read_line(&mut input_task) {
            break len;
        }
        println!("Invalid value, try again");
    };

    println!(
        "write the commentary for add in the to-do-RList. It's not obligatory. Ex commentary1"
    );
    let mut input_commentary: String = String::new();
    std::io::stdin()
        .read_line(&mut input_commentary)
        .expect("Can not read user input");

    if len_input_task >= 21 {
        input_task = format!("{}.", &input_task[..18]);
        len_input_task = input_task.len();
    }
    let total_space: usize = 20 - len_input_task;
    let mut space_input_task: String = String::new();
    for _i in 1..total_space {
        space_input_task = format!("{} ", space_input_task);
    }
    let content_file = format!(
        "❌   | {}{}| {}\n",
        input_task.trim().bold().blue(),
        space_input_task,
        input_commentary.trim().green()
    );
    table_line.push(content_file);
    table_line[0] = format!(
        "{name_file}.todoR | progression: {}/{}\n",
        nbr_complete,
        table_line.len() - 3
    );
    table_line.iter().for_each(|l| print!("{l}"));

    Some(table_line)
}
