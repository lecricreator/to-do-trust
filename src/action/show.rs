use crate::gestionary_file;

pub fn show(argc: usize, args: &Vec<String>) {
    if argc != 3 {
        println!("Need 2 arguments for show the to-do-rustline.\nArgument 1 : show\nArgument 2 : name_of_the_to-do-rustlist");
    }
    let mut file = match gestionary_file::find_file(args){
        Ok(f) => f,
        Err(e) => {
            println!("{} not exist.\nTap 'list' for see all the to-do-rustfile.{}", &args[2], e);
            return
        }
    };
    let file_content = gestionary_file::read_file(&mut file);
    println!("{file_content}");
}