use std::fs;

fn main() {
    let paths = fs::read_dir(".").unwrap();

    for result in paths {
        let path = result.unwrap().path();
        let metadata = fs::metadata(path.to_str().unwrap()).unwrap();
        let type_ = if metadata.is_dir() { "Dirc" } else { "File" };
        println!("{}: {}", type_, path.display())
    }
}