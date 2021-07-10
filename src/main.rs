use std::fs;
use std::env;
use colored::Colorize;
use gitignore;
use path_absolutize::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
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

    // TODO find safer way to check for gitignore (windows compatible)
    let gi_exists = fs::metadata(args[1].to_owned() + "/.gitignore").is_ok();
    if !gi_exists {
        println!("{}", "WRN: .gitignore not found, may output TODOs in dependencies".yellow())
    }

    let mut todos = vec!();

    if gi_exists {
        let gi_str = args[1].to_owned() + "/.gitignore";
        let _p = std::path::Path::new(&gi_str).absolutize().unwrap();
        let gi_str_abs = _p.to_str().unwrap();
        let gi_path = std::path::Path::new(&gi_str_abs);
        let gi= gitignore::File::new(gi_path).unwrap();
        iterate_included_files(&mut todos, &gi).unwrap();
    } 
    else {
        traverse_dir(&args[1], &mut todos).unwrap();
    }

    // TODO eventually add option to export to file
    println!("{:4} {:05} {:20} {:50}", "", "LINE".bold(), "FILE".bold(), "COMMENT".bold());
    for todo in todos {
        println!("{:4} {:05} {:20} {:50}", "TODO".bold(), todo.line_number.to_string().yellow(), todo.file_name.green(), todo.comment);
    }
}

fn traverse_dir(path: &str, todos: &mut Vec<Todo>) -> Result<(), std::io::Error> {    
    let objects = fs::read_dir(path.to_owned())?;

    for result in objects {
        let obj_path = result?.path();
        let obj_str = obj_path.to_str().unwrap();
        let obj_metadata = fs::metadata(obj_str)?;

        if obj_metadata.is_dir() {
            traverse_dir(obj_str, todos)?;
        }
        else if obj_metadata.is_file() && obj_str.ends_with(".py") { // TODO support other files besides .py
            get_todos(obj_str, todos)?;
        }
    }
    
    Ok(())
}

fn iterate_included_files(todos: &mut Vec<Todo>, gi: &gitignore::File) -> Result<(), std::io::Error> {
    for file_path in gi.included_files().unwrap() {
        let file_str = file_path.to_str().unwrap();
        if file_str.ends_with(".py") { // TODO support other files besides .py
            get_todos(file_str, todos)?;
        }
    }
    
    Ok(())
}

fn get_todos(path: &str, todos: &mut Vec<Todo>) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let mut line_number = 0;
    let file_name = path.split("/").last().unwrap();

    for line in contents.lines() {
        line_number += 1;
        if line.contains("TODO") {
            let (_, comment) = line.split_once("TODO").unwrap();
            let cleaned_comment = comment.replace(":", "").trim().to_owned();
            todos.push(Todo::new(cleaned_comment, file_name.to_owned(), line_number))
        }
    }

    Ok(())
}

struct Todo {
    comment: String,
    file_name: String,
    line_number: u32
}

impl Todo {
    fn new(comment: String, file_name: String, line_number: u32) -> Todo {
        Todo {
            comment: comment,
            file_name: file_name,
            line_number: line_number
        }
    }
}