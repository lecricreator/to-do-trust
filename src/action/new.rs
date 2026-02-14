use crate::errors;
use crate::manage_file;

pub fn new_action(args: &[String]) -> Result<(), errors::MyError> {
    let Some(file_name) = args.first() else {
        return Err(errors::MyError::FileNotExist);
    };
    manage_file::create_file(file_name)?;
    Ok(())
}