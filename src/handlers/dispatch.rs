use python_h;

pub fn dispatch(file: Vec<char>, type: String) -> i32 {
    let status: i32 = 0;

    if type == "python3" || type == "python" {
        status = match python_h::python_h(file) {
            Ok(ret) => {
                println("Python exec status: {}", ret);
            }
            Err(e) {
                println!("Error: {}", e);
                process::exit(84);
            }
        }
    }
}