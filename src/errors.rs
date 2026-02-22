use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrFile {
    #[error("Canno't open the file.")]
    ConnotOpenFile,
    #[error("Canno't remove the file.")]
    ConnotRemoveFile,
    #[error("This file already exist.")]
    FileAlreadyExist,
    #[error("Not enough argument. Filename needed.")]
    FileNotExist,
}

#[derive(Error, Debug)]
pub enum ErrInput {
    #[error("Bad input")]
    BadInput,
    #[error("Input task is empty.")]
    InputTaskEmpty,
    #[error("value out of index.")]
    ValueOutIndex,
}

#[derive(Error, Debug)]
pub enum ErrArg {
    #[error("Need an action to modify the to-do-trust")]
    ArgNeedAction,
    #[error("Need a file to modify the to-do-trust.")]
    ArgNeedFile,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error occurred.")]
    IoError(#[from] std::io::Error),
    #[error("varrerror env")]
    EnvError(#[from] std::env::VarError),
    #[error("Error of file.")]
    ErrFile(#[from] ErrFile),
    #[error("Error of input.")]
    ErrInput(#[from] ErrInput),
    #[error("Error of argument.")]
    ErrArg(#[from] ErrArg),
    #[error("Canno't read the directory.")]
    ReadDirectory,
    #[error("Len of the task is too big.")]
    ActionNotExist,
    #[error("This is already complete.")]
    AlreadyComplete,
    #[error("This is already uncomplete.")]
    AlreadyUncomplete,
    #[error("Dev make a mistake in function rename. Let a file replace_file in your current derectory. You can remove it.")]
    ReplaceFile,
}