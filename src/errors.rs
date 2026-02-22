use thiserror::Error;

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