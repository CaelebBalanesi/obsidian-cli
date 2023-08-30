mod todo;
mod file;

fn main() {
    println!("Welcome to obsidian-cli");
    println!("[0] Todo");
    let mut option = String::new();
    std::io::stdin().read_line(&mut option)
        .expect("can not read user input.");
    if option.trim() == "0".to_string(){
        todo::start();
    }
}
