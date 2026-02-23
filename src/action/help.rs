use colored::Colorize;

pub fn help_action() {

    println!("To-do-(t)rust-list is a to-do-list in the terminal.\n\nYou can use todotrust in all folder you want.\nThe file is create in the '~/todotrust' file.\n");
    
    println!("{} : Create a new to-do-trust.
            {} {} {}.\n", "new        ".green(), "todotrust".yellow(), "new".green(), "name_of_todoR_file".blue());

    println!("{}        : list allows you to see all the todotrust file you create.
            {} {}\n", "list".green(), "todotrust".yellow(), "list".green());

    println!("{}         : Add a new task to complete and (optionnal) add a commentary for mor details.
            {} {} {}\n", "add".green(), "todotrust".yellow(), "add".green(), "name_of_todoR_file".blue());

    println!("{}        : Show inside the todotrust file for see the progression.
            {} {} {}\n", "show".green(), "todotrust".yellow(), "show".green(), "name_of_todoR_file".blue());

    println!("{}    : complete a task uncomplete.
            {} {} {}\n", "complete".green(),"todotrust".yellow(), "complete".green(), "name_of_todoR_file".blue());

    println!("{}  : uncomplete a task you've already complete. After you can choose the task at remove.
            {} {} {}\n", "uncomplete".green(), "todotrust".yellow(), "uncomplete".green(), "name_of_todoR_file".blue());

    println!("{}      : remove a task, after you can choose the task at remove.
            {} {} {}\n", "remove".green(), "todotrust".yellow(), "remove".green(), "name_of_todoR_file".blue());

    print!("{}      : delete allows you to delete a todotrust.
            {} {} {}\n", "delete".green(), "todotrust".yellow(), "delete".green(), "name_of_todoR_file".blue());
}