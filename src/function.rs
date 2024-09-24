use std::vec;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::the_list::{TaskList, PATH_FILE, PATH_TRASH, output_name};

fn rewrite_file(pathfile: &str, v : &Vec<TaskList>) {
    let file = File::create(pathfile).expect("FileError: failed to found file.");
    let mut writer = BufWriter::new(file);
    let _= serde_json::to_writer(&mut writer, &v).expect("WriteError: file failed to write.");
    let _= writer.flush();
}

pub fn add(item : &[String]){
    let name = item[0].to_owned();
    let deadline = item[1].to_owned();
    let mut task_list = TaskList::new(PATH_FILE);
    let new_task = TaskList {name, state:false, deadline : Some(deadline)};
    task_list.push(new_task);
    rewrite_file(PATH_FILE, &task_list);
}

pub fn edit(item : &[String]){
    let old_name = item[0].to_owned();
    let name = item[1].to_owned();
    let deadline = item[2].to_owned();
    let mut task_list = TaskList::new(PATH_FILE);
    let new_task = TaskList {name, state:false, deadline : Some(deadline)};
    for (i, task) in task_list.iter().enumerate() {
        if task.name == old_name {
            task_list[i] = new_task;
            break;
        }
    }
    rewrite_file(PATH_FILE, &task_list);
}

pub fn done(nums : &[String]) {
    let mut task_list = TaskList::new(PATH_FILE);
    for num in nums {
        task_list[num.to_owned().parse::<usize>().unwrap()-1].state = true;
    }
    rewrite_file(PATH_FILE, &task_list);
}

pub fn remove(nums : &[String]) {
    let mut task_list = TaskList::new(PATH_FILE);
    for num in nums {
        task_list.remove(num.to_owned().parse::<usize>().unwrap()-1);
    }
    rewrite_file(PATH_FILE, &task_list);
}

pub fn reset() {
    let task_list = TaskList::new(PATH_FILE);
    rewrite_file(PATH_TRASH, &task_list);
    let task_list : Vec<TaskList> = vec![];
    rewrite_file(PATH_FILE, &task_list);
}

pub fn restore() {
    let task_list = TaskList::new(PATH_TRASH);
    rewrite_file(PATH_FILE, &task_list);
}

pub fn sort() {
    let mut task_list = TaskList::new(PATH_FILE);
    task_list.sort_by(|x, y| i32::from(x.state).cmp(&i32::from(y.state)).then(x.deadline.as_ref().unwrap()[..4].cmp(&y.deadline.as_ref().unwrap()[..4])).then(x.deadline.as_ref().unwrap()[5..7].cmp(&y.deadline.as_ref().unwrap()[5..7])).then(x.deadline.as_ref().unwrap()[8..].cmp(&y.deadline.as_ref().unwrap()[8..])));
    rewrite_file(PATH_FILE, &task_list);
}

pub fn row(item : &String) {
    if item != "todo" && item != "done" { panic!("InputError: input error with option row. Please with the parameter 'todo' or 'done.'") };
    let task_list = TaskList::new(PATH_FILE);
    let mut row_list : Vec<TaskList> = vec![];
    for task in task_list {
        if task.state == (item != "todo") {
            row_list.push(task);
        }
    }
    output_name(row_list);
}

pub fn help() {
    let file_content = include_str!(".././help.txt");
    println!("{file_content}");
}