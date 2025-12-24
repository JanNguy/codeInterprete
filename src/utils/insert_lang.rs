use std::collections::HashMap;

pub fn insert_lang() -> HashMap<String, String> {
    let mut buf = HashMap::new();

    buf.insert("#!/usr/bin/env python3".to_string(), "python3".to_string());
    buf.insert("#!/usr/bin/env python".to_string(), "python".to_string());
    buf.insert("#!Rust".to_string(), "rust".to_string());
    buf.insert("#!C".to_string(), "C".to_string());
    buf.insert("#!CPP".to_string(), "CPP".to_string());
    return buf;
}
