# to-do-listing
Create my own to-do-list in Rust for learnig Rust.

Question :
- cherche à convertir i32 par usize pour faire boucle for. C'est quoi try_into et unwrap (remove.rs)Ex :<br/>
```RS
    let input_index_err:i32;
    let table_line:Vec<String>;
    (input_index_err, table_line) = show_and_select_index(file);
    if input_index_err == -1 {return Ok(());}
    let input_index:usize = input_index_err.try_into().unwrap();
```
- A quel moment on met le pointeur (*), pourquoi à ce moment là ? C'est la même choses que C (une adresse) : <br/>
```RS
fn remove_line(table_line:Vec<String>, file_at_replace:File, input_index:usize, t: &usize) {
    if *t != input_index + 3 { // pourquoi que maintenant et pas après ?
        file_at_replace.write(table_line[t].as_bytes()).expect("Can not write in file");
    }
}
```

PROBLEME :
- When make t.todoR create a file t.todoR.todoR, not verified t.todoR
- If ther's too many words in the add in task it panik.
- add a progression list.