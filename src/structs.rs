#[derive(Debug)]
pub struct Context {
    pub path: String,
    pub file: Vec<char>,

    pub shebang: String,
    pub lang: String,

    pub args: Vec<String>,
}
