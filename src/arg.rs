use crate::gestionary_file;
use crate::new;

pub fn start_program(argc: usize, args: &Vec<String>){
    let action = &args[1];
    if action == "new" {
        new::new_action(argc, args);
    }else if action == "add" {
        println!("work in progress !!!");
    }else if action == "show" {
        if argc == 3 {
            let complete_file_name = format!("{}.todoR", &args[2]);
            let fd = gestionary_file::read_file(&complete_file_name);
            let file_content = match fd {
                Err(e) => {
                    let error_content = e.to_string();
                    if error_content == "No such file or directory (os error 2)"{
                        println!("This to-do-rustline doesn't exist.\nCreate a new to-do-rustline file, example :\nto-do-rustline, new, name_of_file")
                    }else {
                        println!("{e}");
                    } 
                    return;},
                Ok(s) => s,
            };
            println!("{file_content}");
        }
        else {
            println!("Need 2 arguments for show the to-do-rustline.\nArgument 1 : show\nArgument 2 : name_of_the_to-do-rustlist");
        }
    }else {
        println!("This command doesn't exist in the to-do-rustline.");
    }

}