use crate::action::add;
use crate::action::complete;
use crate::action::delete;
use crate::action::help;
use crate::action::list;
use crate::action::new;
use crate::action::remove;
use crate::action::show;
use crate::action::uncomplete;
use crate::errors;


pub fn start_program(args: &[String]) -> Result<(), errors::MyError> {
    let (action, args) = args.split_first().ok_or_else(|| errors::MyError::ActionNeeded)?;
    match action.as_str() {
        "new" => new::new_action(args)?,
        "show" => show::show_action(args)?,
        "add" => add::add_action(args)?,
        "help" => help::help_action(),
        "remove" => remove::remove_action(args)?,
        "complete" => complete::complete_action(args)?,
        "uncomplete" => uncomplete::uncomplete_action(args)?,
        "delete" => delete::delete_action(args)?,
        "list" => list::list_action()?,
        _ => Err(errors::MyError::ActionNotExist)?,
    }
    Ok(())
}
