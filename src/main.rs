use std::fs;
use std::env;
use colored::Colorize;

// \x1b is used to print in red

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

    let objects = fs::read_dir(args[1].to_owned()).unwrap();
    for result in objects {
        let object = result.unwrap().path();
        let metadata = fs::metadata(object.to_str().unwrap()).unwrap();
        let type_ = if metadata.is_dir() { "Dirc" } else { "File" };
        println!("{}: {}", type_, object.display())
    }
}