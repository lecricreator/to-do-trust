pub fn help_action() {
    println!("To-do-RList is a to-do-list in the terminal.\n");
    println!("new        : Create a new to-do-RList.
            todoRList new name_of_todoRList_file.\n");
    println!("add        : Add a new task to complete and (optionnal) add a commentary for mor details.
            todoRList add name_of_todoRList_file write_task (optionnal)write_comentary\n");
    println!("show       : Show the todoRList for see the progression.
            todoRList show name_of_todoRList_file\n");
    println!("remove     : remove a task, after you can choose the task at remove.
            todoRList remove name_of_todoRList_file\n");
    println!("complete   : complete a task.
            todoRList complete name_of_todoRList_file\n");
    println!("uncomplete : uncomplete a task. After you can choose the task at remove.
            todoRList uncomplete name_of_todoRList_file\n");
    println!("list        : list permit you to see all the todoRList file you create.
            todoRList file\n");
    println!("delete      : delete permit you to delete a todoRList.
            todoRList delete name_of_todoRList_file\n");
}