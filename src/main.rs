use std::fs;
mod utils;

fn main() {
    let file: Vec<char> = fs::read_to_string("./file").unwrap().chars().collect();

    match utils::get_shebang::get_shebang(file) {
        Ok(shebang) => println!("{:?}", shebang),
        Err(e) => println!("Error: {}", e),
    }
}