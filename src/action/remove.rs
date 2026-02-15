use crate::HEADER_SIZE;
use crate::errors;
use crate::manage_file;
use std::fs::File;
use std::io::Write;

pub fn remove_action(args: &[String]) -> Result<(), errors::MyError> {
    let file_name: &String = args.first().ok_or_else(|| errors::MyError::ActionNeeded)?;
    manage_file::replace_file(file_name, remove_line, "remove")?;
    Ok(())
}

fn remove_line(
    table_line: &Vec<String>,
    mut file_at_replace: &File,
    input_index: usize,
    t: &usize,
) -> Result<(), errors::MyError>{
    if *t != input_index + HEADER_SIZE {
        file_at_replace.write(table_line[*t].as_bytes())?;
        print!("{}", table_line[*t]);
    }
    Ok(())
}
