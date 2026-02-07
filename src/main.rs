mod arg;
pub mod gestionary_file;
mod action{
    pub mod new;
    pub mod show;
    pub mod add;
    pub mod help;
    pub mod remove;
    pub mod complete;
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let argc = if args.len() <= 1 {
        println!("To-do-rustline need minimum 1 argument for execute the program.");
        return
    }else{
        args.len()
    };
    arg::start_program(argc, &args);
}