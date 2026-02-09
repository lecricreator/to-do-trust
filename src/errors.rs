pub const FILE_NOT_FOUND: &str = ".todoR file '{}' not exist.\nTap 'list' for see all the .todoR file.";
pub const NO_SUFFISALY_ARGS: &str = "Need {} arguments.\n1: todoR \n2: action\n3. name of file";
//"todorustlist add name_of_todorustlist_file write_task (optionnal)write_comentary"
#[derive(PartialEq)]
pub enum ErrorName {
        ErrFileNotFound,
        ErrNotSuffisalyArg2,
        ErrNotSuffisalyArg3,
        ErrNotSuffisalyArg5,
}

pub fn print_error (err: ErrorName, err_msg: String) {
    if err == ErrorName::ErrFileNotFound {
        eprintln!(".todoR file '{}.todoR' not exist or cannot be read or modified.\nTap 'list' for see all the .todoR file and verified if it exist.", err_msg)
    }else if err == ErrorName::ErrNotSuffisalyArg2 {
        eprintln!("Need {} arguments.\n1: todoR file \n2: action", err_msg);
    }else if err == ErrorName::ErrNotSuffisalyArg3 {
        eprintln!("Need {} arguments.\n1: todoR file \n2: action\n3. name of .todoR file", err_msg);
    }else if err == ErrorName::ErrNotSuffisalyArg5 {
        eprintln!("Need {} arguments.\n1: todoR file \n2: add\n3. name of.todoR file\n4. write_task\n5. (optionnal)write_comentary", err_msg);
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