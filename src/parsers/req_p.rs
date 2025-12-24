use std::collections::HashMap;
use crate::utils::print_supported;

pub fn req_p(map_l: &HashMap<String, String>, shebang: String) -> String {
    match map_l.get(&shebang) {
        Some(value) => value.clone(),
        None => {
            print_supported::print_supported(&map_l);
            String::from("Error: not available")
        },
    }
}