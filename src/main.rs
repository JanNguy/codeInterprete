use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

mod utils;
mod parsers;
mod structs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        process::exit(84);
    }

    let file_content = match fs::read_to_string(&args[1]) {
        Ok(content) => content.chars().collect(),
        Err(e) => {
            println!("Error reading file: {}", e);
            process::exit(84);
        }
    };

    let mut c = structs::Context {
        file: file_content,
        lang: String::new(),
        path: args[1].clone(),
        shebang: String::new(),
        args: args.clone(),
    };

    let shebang_result = utils::get_shebang::get_shebang(c.file.clone());

    c.shebang = match shebang_result {
        Ok(shebang) => {
            println!("{:?}", shebang);
            shebang
        },
        Err(_) => {
            match utils::get_ext::get_ext(&c.path) {
                Some(ext) => {
                    println!("{:?}", ext);
                    ext
                },
                None => {
                    println!("Error: Could not get extension");
                    process::exit(84);
                },
            }
        },
    };

    let lang: HashMap<String, String> = utils::insert_lang::insert_lang();
    let status = parsers::req_p::req_p(&lang, c.shebang);

    if status.contains("Error: ") {
        process::exit(84);
    }
}