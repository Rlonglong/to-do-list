use std::env;
use todo::function::*;
use todo::the_list::{output_name, TaskList, PATH_FILE};


fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() > 1{
        match args[1].as_str() {
            "add" => add(&args[2..]),
            "edit" => edit(&args[2..]),
            "done" => done(&args[2..]),
            "rm" => remove(&args[2..]),
            "reset" => reset(),
            "restore" => restore(),
            "sort" => sort(),
            "raw" => {raw(&args[2]); return;},
            "--help" => {help(); return;},
            "list" => (),
            &_ => panic!("TypeError: input some paramter that didn't find. If you need help, plese run 'todo --help' to apply suggestion."),
        }
    }

    output_name(TaskList::new(PATH_FILE));
}