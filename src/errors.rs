use colored::Colorize;
use thiserror::Error;

pub const FILE_NOT_FOUND: &str = ".todoR file '{}' not exist.\nTap 'list' for see all the .todoR file.";
pub const NO_SUFFISALY_ARGS: &str = "Need {} arguments.\n1: todoR \n2: action\n3. name of file";
//"todorustlist add name_of_todorustlist_file write_task (optionnal)write_comentary"
#[derive(PartialEq)]
pub enum ErrorName {
        ErrFileNotFound,
        ErrReadDirectory,
        ErrLenTooBig,
        ErrActionNotExist,
        ErrNotSuffisalyArg(usize),
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Not enough argument. Filename needed.")]
    FileNotExist,
    #[error("This file already exist.")]
    FileAlreadyExist,
    #[error("Their not suffisaly argument.")]
    NotSuffisalyArg,
    #[error("Not enough argument. Action needed.")]
    ActionNeeded,
    #[error("Canno't read the directory.")]
    ReadDirectory,
    #[error("Bad input")]
    BadInput,
    #[error("Len of the task is too big.")]
    LenTooBig,
    #[error("This action does'nt exist. Whrite help for have all action.")]
    ActionNotExist,
    #[error("Canno't open the file.")]
    ConnotOpenFile,
    #[error("Canno't remove the file.")]
    ConnotRemoveFile,
    #[error("This is already complete.")]
    AlreadyComplete,
    #[error("This is already uncomplete.")]
    AlreadyUncomplete,
    #[error("It cannot parse to another type.")]
    CannotParse,
    #[error("Input task is empty.")]
    InputTaskEmpty,
    #[error("Dev make a mistake in function rename. Let a file replace_file. You can remove it.")]
    ReplaceFile,
    #[error("value out of index.")]
    ValueOutIndex,
    #[error("I/O error occurred.")]
    IoError(#[from] std::io::Error),
    #[error("varrerror env")]
    EnvError(#[from] std::env::VarError),
}

pub fn print_error (err: ErrorName, err_msg: String) {
    if err == ErrorName::ErrFileNotFound {
        let red_print = format!(".todoR file '{}.todoR' not exist or cannot be read or modified.\n", err_msg).red();
        eprintln!("{}Tap 'list' for see all the .todoR file and verified if it exist.", red_print);
    }else if err == ErrorName::ErrReadDirectory {
        let red_print = format!("Error reading directory: {}.\n", err_msg).red();
        eprintln!("{}", red_print);
    }else if err == ErrorName::ErrNotSuffisalyArg(3) {
        let red_print = format!("Need {} arguments.\n", err_msg).red();
        eprintln!("{}1: todoR \n2: action\n3. name of todoR file", red_print);
    }else if err == ErrorName::ErrNotSuffisalyArg(5) {
        let red_print = format!("Need {} arguments.\n", err_msg).red();
        eprintln!("{}1: todoR \n2: add\n3. name of.todoR file\n4. write_task\n5. (optionnal)write_comentary", red_print);
    }
}

pub fn verified_arg(argc: usize, nbr_arg: usize) -> bool {
    let conv_to_str:String = nbr_arg.to_string();
    if argc != nbr_arg {
        if nbr_arg == 2 {
            print_error(ErrorName::ErrNotSuffisalyArg(2), conv_to_str);
        } else if nbr_arg == 3 {
            print_error(ErrorName::ErrNotSuffisalyArg(3), conv_to_str);
        } else if nbr_arg == 5 {
            print_error(ErrorName::ErrNotSuffisalyArg(5), conv_to_str);
        }
        return false
    }
    return true
}