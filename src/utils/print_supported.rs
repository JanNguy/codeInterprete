use std::collections::HashMap;

pub fn print_supported(map_l: &HashMap<String, String>) {
    println!("Supported Languages and their shebangs: ");
    for (key, value) in map_l.iter() {
        println!("{}: {}", value, key);
    }
    println!("For the moment custom languages are not supported!");
}
