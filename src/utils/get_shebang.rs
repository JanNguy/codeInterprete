pub fn get_shebang(file: Vec<char>) -> Result<String, String> {
    let mut shebang;

    if file.len() < 2 {
        return Err("Fichier trop court pour un shebang".to_string());
    }
    if file[0] != '#' || file[1] != '!' {
        return Err("Shebang non valide (#! manquant)".to_string());
    }
    shebang = String::new();
    for i in 0..file.len() {
        if file[i] == '\n' {
            break;
        }
        shebang.push(file[i]);
    }
    if shebang.len() <= 2 {
        return Err("Shebang incomplet".to_string());
    }
    Ok(shebang)
}
