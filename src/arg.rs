use crate::action::new;
use crate::action::show;
use crate::action::add;
use crate::action::help;
use crate::action::remove;
use crate::action::complete;
use crate::action::uncomplete;
use crate::action::delete;

pub fn start_program(argc: usize, args: &Vec<String>){
    let action = &args[1];
    if action == "new" {
        new::new_action(argc, args);
    }else if action == "show" {
        show::show(argc, args);
    }else if action == "add" {
        let _ = add::add(argc, args);
    }else if action == "help" {
        help::help();
    }else if action == "remove" {
        remove::remove(argc, args);
    }else if action == "complete" {
        complete::complete(argc, args);
    }else if action == "uncomplete" {
        uncomplete::uncomplete(argc, args);
    }else if action == "file" {
        delete::delete(argc, args);
    }else if action == "delete" {
        delete::delete(argc, args);
    }else {
        println!("This command doesn't exist in the to-do-rustline.");
    }

}