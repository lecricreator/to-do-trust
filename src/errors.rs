use colored::Colorize;

pub const FILE_NOT_FOUND: &str = ".todoR file '{}' not exist.\nTap 'list' for see all the .todoR file.";
pub const NO_SUFFISALY_ARGS: &str = "Need {} arguments.\n1: todoR \n2: action\n3. name of file";
//"todorustlist add name_of_todorustlist_file write_task (optionnal)write_comentary"
#[derive(PartialEq)]
pub enum ErrorName {
        ErrFileNotFound,
        ErrReadDirectory,
        ErrNotSuffisalyArg2,
        ErrNotSuffisalyArg3,
        ErrNotSuffisalyArg5,
}

pub fn print_error (err: ErrorName, err_msg: String) {
    if err == ErrorName::ErrFileNotFound {
        let red_print = format!(".todoR file '{}.todoR' not exist or cannot be read or modified.\n", err_msg).red();
        eprintln!("{}Tap 'list' for see all the .todoR file and verified if it exist.", red_print);
    }else if err == ErrorName::ErrNotSuffisalyArg5 {
        let red_print = format!("Need {} arguments.\n", err_msg).red();
        eprintln!("{}1: todoR \n2: add\n3. name of.todoR file\n4. write_task\n5. (optionnal)write_comentary", red_print);
    }else if err == ErrorName::ErrReadDirectory {
        let red_print = format!("Error reading directory: {}.\n", err_msg).red();
        eprintln!("{}", red_print);
    }else if err == ErrorName::ErrNotSuffisalyArg3 {
        let red_print = format!("Need {} arguments.\n", err_msg).red();
        eprintln!("{}1: todoR \n2: action\n3. name of todoR file", red_print);
    }else if err == ErrorName::ErrNotSuffisalyArg5 {
        let red_print = format!("Need {} arguments.\n", err_msg).red();
        eprintln!("{}1: todoR \n2: add\n3. name of.todoR file\n4. write_task\n5. (optionnal)write_comentary", red_print);
    }
}

pub fn verified_arg(argc: usize, nbr_arg: usize) -> bool {
    let conv_to_str:String = nbr_arg.to_string();
    if argc < nbr_arg {
        if nbr_arg == 2 {
            print_error(ErrorName::ErrNotSuffisalyArg2, conv_to_str);
        } else if nbr_arg == 3 {
            print_error(ErrorName::ErrNotSuffisalyArg3, conv_to_str);
        } else if nbr_arg == 4 {
            print_error(ErrorName::ErrNotSuffisalyArg5, conv_to_str);
        }
        return false
    }
    return true
}