use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;
use std::io::Write;
pub const PATH_FILE : &str= "task_list.json";
pub const PATH_TRASH : &str= "trash.json";



fn read_as_vec(file_path: &str) -> Vec<TaskList> {
    let json_file_path = Path::new(file_path);
    let file = match File::open(json_file_path){
        Ok(file) => file, 
        Err(_) => {
            match File::create(json_file_path) {
                Err(why) => panic!("FileError: failed to create file {:?}.", why),
                Ok(file) => file,
            }
        }
    };
    let reader = BufReader::new(file);
    let users : Vec<TaskList> = match serde_json::from_reader(reader) {
        Ok(user) => user, 
        Err(_) => return vec![],
    };
    users
}

pub fn output_name(vec : Vec<TaskList>) {
    let mut s = "".to_string();
    for (i, task) in vec.iter().enumerate() {
        s += &format!("{}.{}\t{}\t{}\n", 
            i+1, task.name, task.deadline.as_ref().unwrap(), 
            if task.state {"✅"} else {"❌"});
    }
    let mut tw = TabWriter::new(vec![]);
    write!(&mut tw, "{s}").unwrap();
    tw.flush().unwrap();

    let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    println!("{}", written)
}

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub name: String,
    pub state: bool,
    pub deadline: Option<String>,
}
impl TaskList {
    pub fn new(path : &str) -> Vec<TaskList> {
        return read_as_vec(path);
    }
}