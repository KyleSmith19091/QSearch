use std::path::PathBuf;

pub trait Index {
    fn query(&self, query_words: &Vec<String>) -> Vec<(&PathBuf, f64)>;
    fn handle_token(&mut self, path: &PathBuf, token: String);
    fn to_index_file(&self, path: &str);
    fn from_index_file(path: &str) -> Self;
}
