use std::fs::OpenOptions;
use std::io::Write;
use crate::the_list::{file_2_vec, PATH_FILE};

pub fn edit(index : usize, task : &str){
    let mut file = OpenOptions::new()
        .write(true)
        .open(PATH_FILE)
        .expect("cannot open file");

    let vec_of_list = file_2_vec(PATH_FILE.to_string());
    
    for (i, item) in vec_of_list.iter().enumerate() {
        if i != 0{
            file
                .write_all("\n".as_bytes())
                .expect("write failed");
        }
        if i == index-1 {
            file
                .write_all(task.as_bytes())
                .expect("write failed");
        }
        else {
            file
                .write_all(item.as_bytes())
                .expect("write failed");
        }
    }

}