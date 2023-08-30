use chrono;

use super::file;

pub fn start(){
    println!("Welcome to obsidian-cli/todo");
    println!("[0] new todo");
    println!("[1] list todo");
    println!("[2] complete todo");
    println!("[3] delete todo");
    let mut option = String::new();
    std::io::stdin().read_line(&mut option)
        .expect("can not read user input.");

    if option.trim() == "0"{
        new_todo();
    }else if option.trim() == "1"{
        list_todo();
    }
}

fn new_todo(){
    println!("What is the title of the todo?");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title)
        .expect("can not read user input.");
    println!("When is the due date for the todo?(\"MM/DD/YYYY\")");
    let mut due_date = String::new();
    std::io::stdin().read_line(&mut due_date)
        .expect("can not read user input.");
    let creation_date = chrono::offset::Utc::now();
    let mut entry: String = Default::default();
    entry.push_str("|");
    entry.push_str(&creation_date.format("%m/%d/%y").to_string());
    entry.push_str("|");
    entry.push_str(&title.trim());
    entry.push_str("|");
    entry.push_str(&due_date.trim());
    entry.push_str("|   |\n");
    file::add_todo(entry.to_string());
}

fn list_todo(){
    let data = file::get_todo();
    println!("{data}")
}
