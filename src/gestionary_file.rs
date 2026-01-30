use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn read_file(str_path: &String) -> Result<String, io::Error>{
    let mut fd = File::open(str_path)?;
    let mut content: String = String::new();
    fd.read_to_string(&mut content)?;
    Ok(content)
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
    let _ = writeln!(file, "{}", &name_file).expect("Cannot write in the file {total_name_file}.");
}