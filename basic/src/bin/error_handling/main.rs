use std::path::Path;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("The file name to be pased in");

    let cwd = std::env::current_dir().unwrap();

    println!("cwd {}", cwd.display());

    let current_file = file!();
    let current_dir = Path::new(current_file).parent().unwrap();
    let dir_name = current_dir.display();

    println!("Current directory: {}", current_dir.display());

    let file_path = dir_name.to_string() + "/" + &file_name;

    println!("full file path {}", &file_path);

    let file = std::fs::read_to_string(file_path).expect("unable to read the file as string");

    file.lines().for_each(|line| println!("{}", line));
}
