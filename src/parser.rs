use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct task {
    pub name: String,
    pub state: bool,
    pub deadline: Option<String>,
}

pub fn print_an_address() -> Result<()> {
    // Some data structure.
    /*let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };*/

    // Serialize it to a JSON string.
    //let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    //println!("{}", j);

    Ok(())
}

let new_task = TaskList {name : "hahaha".to_string() , state:false, deadline : Some("2024".to_string())};
    let task_list = vec!(new_task);
    print!("{}", task_list[0].name);
    let file = File::create(PATH_FILE).expect("file not found");
    let mut writer = BufWriter::new(file);

    let _= serde_json::to_writer(&mut writer, &task_list).expect("file not write");
    let _= writer.flush();

    let reader = BufReader::new(file);
    let users : Vec<TaskList> = match serde_json::from_reader(reader) {
        Ok(user) => user, 
        Err(why) => panic!("couldn't read {:?}", why),
    };
    println!("{}", users[0].name);
}