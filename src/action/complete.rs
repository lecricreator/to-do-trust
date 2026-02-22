use std::{io::{Write}};
use std::fs::{File};
use crate::{HEADER_SIZE, manage_file::{self}};
use crate::errors::{self};

pub fn complete_action(args: &[String]) -> Result<(), errors::MyError> {
    let file_name: &String = args.first().ok_or_else(|| errors::MyError::ErrArg(errors::ErrArg::ArgNeedFile))?;
    manage_file::replace_file(file_name, complete_file, "complete task")?;
    Ok(())
}

fn complete_file(table_line: &Vec<String>, mut file_at_replace: &File, input_index:usize, t: &usize) -> Result<(), errors::MyError>{
    if *t == input_index + HEADER_SIZE {
        let modify_str = table_line[*t].replace("❌", "✅");
        file_at_replace.write(modify_str.as_bytes())?;
        print!("{}", modify_str);
    }else{
        print!("{}", table_line[*t]);
        file_at_replace.write(table_line[*t].as_bytes())?;
    }
    Ok(())
}