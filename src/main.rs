use std::io::{self, Write};
use std::{fs::{File, OpenOptions}, path::PathBuf};

extern crate dirs;

fn get_todo_dir() -> PathBuf {
    dirs::data_local_dir().expect("Could not find local data dir")
}

fn setup_todo_dir() -> PathBuf {
    let todo_dir = get_todo_dir();

    if todo_dir.exists() {
        let _ = open_or_create_todo_file("general.txt");
    } else {
        std::fs::create_dir_all(&todo_dir).expect("Failed to create general directory");
    }

    todo_dir
}

fn create_file_if_not_exists(file_name: &str) -> io::Result<File> {
    
}

fn open_or_create_todo_file(file_name: &str) -> io::Result<File> {
    let todo_dir = setup_todo_dir();
    let file_path = todo_dir.join(file_name);

    let file = if file_path.exists() {
        println!("File {:?} already created", file_name);
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path)?
    } else {
        File::create("foo.txt")?
    };

    Ok(file)
}

fn main() {
    let todo_dir = setup_todo_dir();
}
