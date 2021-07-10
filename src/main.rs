use std::fs;
use std::env;
use colored::Colorize;
use gitignore;

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

    // TODO: find safer way to check for gitignore
    let gi_exists = fs::metadata(args[1].to_owned() + "/.gitignore").is_ok();
    if !gi_exists {
        println!("{}", "WRN: .gitignore not found, may output TODOs in dependencies".yellow())
    }

    let mut todos = vec!();

    if gi_exists {
        let gi_str = args[1].to_owned() + "/.gitignore";
        let gi_path = std::path::Path::new(&gi_str);
        let gi= gitignore::File::new(gi_path).unwrap();
        traverse_dir(&args[1], &mut todos, Some(&gi)).unwrap();
    } 
    else {
        traverse_dir(&args[1], &mut todos, None).unwrap();
    }

    for todo in todos {
        println!("TODO: {}", todo.green());
    }
}

fn traverse_dir(path: &str, todos: &mut Vec<String>, gi_option: Option<&gitignore::File>) -> Result<(), std::io::Error> {
    let objects = fs::read_dir(path.to_owned())?;

    for result in objects {
        let obj_path = result?.path();
        let obj_str = obj_path.to_str().unwrap();
        let obj_metadata = fs::metadata(obj_str)?;
        
        let is_excluded = match gi_option {
            Some(gi) => { 
                let dir_path = std::path::Path::new(obj_str);
                gi.is_excluded(dir_path).unwrap()
            }
            None => { false }
        };

        if !is_excluded && obj_metadata.is_dir() {
            traverse_dir(obj_str, todos, gi_option)?;
        }
        else if !is_excluded && obj_metadata.is_file() && obj_str.ends_with(".py") {
            get_todos(obj_str, todos)?;
        }
    }
    
    Ok(())
}

fn get_todos(path: &str, todos: &mut Vec<String>) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(path)?;

    for line in contents.lines() {
        if line.contains("TODO") {
            let (_, comment) = line.split_once("TODO").unwrap();
            let cleaned_comment = comment.replace(":", "").trim().to_owned();
            todos.push(cleaned_comment)
        }
    }

    Ok(())
}