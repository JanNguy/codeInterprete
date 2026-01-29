pub fn get_ext(path: &str) -> Option<String> {
    let filename = if let Some(slash_pos) = path.rfind('/') {
        &path[slash_pos + 1..]
    } else {
        path
    };
    if let Some(dot_pos) = filename.rfind('.') {
        if dot_pos > 0 && dot_pos < filename.len() - 1 {
            return Some(filename[dot_pos + 1..].to_string());
        }
    }
    None
}