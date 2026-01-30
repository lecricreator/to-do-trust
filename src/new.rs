use crate::gestionary_file;

pub fn new_action(argc: usize, args: &Vec<String>) {
    if argc == 3 {
        gestionary_file::create_file(&args[2]);
    }else {
        println!("Need the name of the to-do-rustline for create the file .todoR.")
    }
}