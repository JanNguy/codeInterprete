use std::{collections::{HashMap}, fs, process};
mod utils;
mod parsers;

fn main() {
    let file: Vec<char> = fs::read_to_string("./test/python_shebang").unwrap().chars().collect();
    let mut status: String = String::new();
    let mut lang: HashMap<String, String> = HashMap::new();

    lang = utils::insert_lang::insert_lang();
    let sh = match utils::get_shebang::get_shebang(file) {
        Ok(shebang) => {
            println!("{:?}", shebang);
            shebang
        },
        Err(e) => {
            println!("Error: {}", e);
            process::exit(84);
        },
    };
    status = parsers::req_p::req_p(&lang, sh);
    if status.contains("Error: ") {
        process::exit(84);
    }
    println!("{}", status);
}
