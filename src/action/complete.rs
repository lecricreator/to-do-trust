use crate::gestionary_file::{self};

pub fn complete(argc: usize, args: &Vec<String>){
    if argc != 3{
        println!("Need 3 arguments.\n1: action \n2: task\n3. (optinal) commentary");
    }
    let file = match gestionary_file::find_file(args){
        Ok(f) => f,
        Err(e) => {println!("to-do-rustfile not exist.\nTap 'list' for see all the to-do-rustfile.{}", e); return}
    };
}