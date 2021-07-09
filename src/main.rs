use std::fs;
use std::env;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{}", "ERR: Invalid number of arguments provided".red());
        return
    }

    let is_ok = fs::metadata(args[1].to_owned()).is_ok();
    if !is_ok {
        println!("{}", "ERR: Path provided does not exist".red());
        return
    }

    let is_dir = fs::metadata(args[1].to_owned()).unwrap().is_dir();
    if !is_dir {
        println!("{}", "ERR: Path provided is not a directory".red());
        return
    }

    let todos = traverse_dir(&args[1]).unwrap();
    for todo in todos {
        println!("TODO: {}", todo.green());
    }
}

fn traverse_dir(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut todos = Vec::new();

    let objects = fs::read_dir(path.to_owned()).unwrap();
    for result in objects {
        let obj_path = result.unwrap().path();
        let obj_str = obj_path.to_str().unwrap();
        let obj_metadata = fs::metadata(obj_str).unwrap();

        if obj_metadata.is_dir() {
            println!("DIRC: {}", obj_str.yellow());
            todos.append(&mut traverse_dir(obj_str).unwrap());
        }
        else if obj_metadata.is_file() {
            if obj_str.ends_with(".py") {
                println!("FILE: {}", obj_str.yellow());
                todos.append(&mut get_todos(obj_str).unwrap());
            }
        }
    }
    
    Ok(todos)
}

fn get_todos(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut todos = Vec::new();

    let contents = fs::read_to_string(path).unwrap();
    for line in contents.lines() {
        if line.contains("TODO") {
            let (_, comment) = line.split_once("TODO").unwrap();
            let cleaned_comment = comment.replace(":", "")
                                                .trim()
                                                .to_owned();
            todos.push(cleaned_comment.to_owned())
        }
    }

    Ok(todos)
}