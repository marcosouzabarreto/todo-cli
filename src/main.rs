extern crate dirs;

use clap::Parser;
use std::{fs::{self, File}, path::PathBuf, process::Command};

fn get_todo_dir() -> PathBuf {
    let data_dir = dirs::data_local_dir().expect("Could not find local data dir");
    data_dir.join("todos")
}

fn setup_todo_dir() -> PathBuf {
    let todo_dir = get_todo_dir();

    if !todo_dir.exists() {
        std::fs::create_dir_all(&todo_dir).expect("Failed to create todo data directory");
    }

    todo_dir
}

fn create_file_if_not_exists(
    file_name: &str,
    target_dir: &PathBuf,
) -> Result<PathBuf, std::io::Error> {
    let file_path = target_dir.join(file_name);

    if file_path.exists() {
        return Ok(file_path);
    };

    match File::create(&file_path) {
        Ok(_) => Ok(file_path),
        Err(err) => Err(err),
    }
}

fn get_todo_path(file_name: &str) -> PathBuf {
    let todo_dir = setup_todo_dir();

    let file_path = create_file_if_not_exists(&file_name, &todo_dir)
        .expect("Failed to create or access todo file");

    println!("{:?}", file_path);
    file_path
}

fn create_tmux_tab(tab_name: &str, command: &str) {
    let output = Command::new("tmux")
        .arg("new-window")
        .arg("-n")
        .arg(tab_name)
        .arg(command)
        .output()
        .expect("Failed to create tmux tab");

    if !output.status.success() {
        println!("Error: Failed to create tmux tab");
    }
}

fn delete_file(file_name: &str) {
    let todo_path = get_todo_path(&file_name);
    fs::remove_file(todo_path).expect("Unable to remove file"); 
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    delete: bool,

    #[arg(short, long, default_value = "nvim")]
    editor: String,

    #[arg(short, long, default_value = "general")]
    file: String,
}

fn main() {
    let args = Args::parse();
    let file_path = get_todo_path(&args.file);
    if args.delete {
       return delete_file(&args.file);
    }
    let open_editor_command = format!("{} {:?}", args.editor, file_path);

    create_tmux_tab("Todo", &open_editor_command);
}
