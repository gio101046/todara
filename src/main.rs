use std::ffi::OsString;
use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use colored::Colorize;
use gitignore;
use path_absolutize::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
        println!("{}", "ERR: Invalid number of arguments provided".red());
        return
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        println!("{}", "ERR: Path provided does not exist".red());
        return
    }

    if !path.is_dir() {
        println!("{}", "ERR: Path provided is not a directory".red());
        return
    }

    // TODO find safer way to check for gitignore (windows compatible)
    let gi_exists = path.join(".gitignore").exists();
    if !gi_exists {
        println!("{}", "WRN: .gitignore not found, may output TODOs in dependencies".yellow())
    }

    let mut todos = vec!();
    if gi_exists {
        let gi_path = path.join(".gitignore");
        let gi_path_abs = gi_path.absolutize().unwrap();
        let gi= gitignore::File::new(&gi_path_abs).unwrap();
        iterate_included_files(&mut todos, &gi).unwrap();
    } 
    else {
        traverse_dir(&path, &mut todos).unwrap();
    }

    // TODO eventually add option to export to file
    println!("{:4} {:05} {:20} {:50}", "", "LINE".bold(), "FILE".bold(), "COMMENT".bold());
    for todo in todos {
        println!("{:4} {:05} {:20} {:50}", "TODO".bold(), todo.line_number.to_string().yellow(), todo.file_name.into_string().unwrap().green(), todo.comment);
    }
}

fn traverse_dir(path: &Path, todos: &mut Vec<Todo>) -> Result<(), std::io::Error> {    
    let objects = fs::read_dir(path)?;

    for result in objects {
        let obj_path = result?.path();
        if obj_path.is_dir() {
            traverse_dir(&obj_path, todos)?;
        }
        else if obj_path.is_file() {
            let extension = match obj_path.extension() {
                Some(ext) => { ext.to_owned().into_string().unwrap() }
                None => { String::from("") }
            };

            // TODO support other files besides .py
            if extension == "py" { 
                get_todos(&obj_path, todos)?;
            }
        }
    }
    
    Ok(())
}

fn iterate_included_files(todos: &mut Vec<Todo>, gi: &gitignore::File) -> Result<(), std::io::Error> {
    for obj_path in gi.included_files().unwrap() {
        let extension = match obj_path.extension() {
            Some(ext) => { ext.to_owned().into_string().unwrap() }
            None => { String::from("") }
        };

        // TODO support other files besides .py
        if !obj_path.is_dir() && extension == "py" { 
            get_todos(&obj_path, todos)?;
        }
    }
    
    Ok(())
}

fn get_todos(path: &PathBuf, todos: &mut Vec<Todo>) -> Result<(), std::io::Error> {
    let file_name = path.file_name().to_owned().unwrap();
    let mut line_number = 0;

    for line in fs::read_to_string(path)?.lines() {
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
    file_name: OsString,
    line_number: u32
}

impl Todo {
    fn new(comment: String, file_name: OsString, line_number: u32) -> Todo {
        Todo {
            comment: comment,
            file_name: file_name,
            line_number: line_number
        }
    }
}