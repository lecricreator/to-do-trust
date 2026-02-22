pub fn help_action() {
    println!("To-do-(t)rust(-list) is a to-do-list in the terminal.\n\nYou can use todotrust in all folder you want.\nThe file is create in the '~/todotrust' file if you make the correct export.\n");
    println!("new        : Create a new to-do-RList.
            todotrust new name_of_todotrust_file.\n");
    println!("add        : Add a new task to complete and (optionnal) add a commentary for mor details.
            todotrust add name_of_todotrust_file write_task (optionnal)write_comentary\n");
    println!("show       : Show the todotrust for see the progression.
            todotrust show name_of_todotrust_file\n");
    println!("remove     : remove a task, after you can choose the task at remove.
            todotrust remove name_of_todotrust_file\n");
    println!("complete   : complete a task.
            todotrust complete name_of_todotrust_file\n");
    println!("uncomplete : uncomplete a task. After you can choose the task at remove.
            todotrust uncomplete name_of_todotrust_file\n");
    println!("list        : list permit you to see all the todotrust file you create.
            todotrust file\n");
    println!("delete      : delete permit you to delete a todotrust.
            todotrust delete name_of_todotrust_file\n");
}