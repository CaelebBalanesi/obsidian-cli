use std::{fs::{OpenOptions, File}, io::{Write, Read}};

pub fn add_todo(todo: String){

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("Todo.md")
        .expect("cannot open file");

    file.write(&todo.as_bytes())
        .expect("write failed");
}

pub fn get_todo() -> String{
    let mut file = File::open("Todo.md").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    return file_content;
}
