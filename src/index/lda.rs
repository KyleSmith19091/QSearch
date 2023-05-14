use crate::index::index;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::process::exit;

use serde::{Serialize, Deserialize};
use rust_stemmers::{Algorithm, Stemmer};

#[derive(Serialize, Deserialize)]
pub struct LDA {
    topics: Vec<String>,
    filter: Vec<String>,
    document_term_mat: HashMap<PathBuf, HashMap<String, u64>>
}

impl LDA {
    fn new(topics: Vec<String>, filter: Vec<String>) -> Self {
        Self {
            topics,
            filter,
            document_term_mat: HashMap::<PathBuf, HashMap<String, u64>>::new(),
        }
    }

    fn remove_stop_words<'a>(&'a self, token: &'a String) -> Option<&String> {
        if !self.filter.contains(&token) {
            Some(token)
        } else {
            None
        }
    }

    // Inefficient AF
    fn stem_word(&self, token: &String) -> String {
        let stemmer = Stemmer::create(Algorithm::English);
        stemmer.stem(&token).to_string()
    }

}

impl index::Index for LDA {
    fn query(&self, query_words: &Vec<String>) -> Vec<(&PathBuf, f64)> {
        todo!()
    }

    fn handle_token(&mut self, path: &PathBuf, token: String) {
        // Remove if stop word
        let token = self.remove_stop_words(&token);

        // Ignore token if it is a stop word
        if token.is_none() {
            return;
        }

        // Stem word
        let token = self.stem_word(&token.unwrap());
        
        // Add to index structure
        if let Some(doc) = self.document_term_mat.get_mut(path) { // Check if doc exists
            if let Some(freq) = doc.get_mut(&token) {
                *freq += 1;
            } else {
                doc.insert(token, 1);
            }
        } else { // Doc does not exist so add term
            self.document_term_mat.insert(
                path.to_owned(),
                HashMap::<String, u64>::from([(token,1)])
            );
        }
    }

    fn to_index_file(&self, path: &str) {
        let file = File::create(path).unwrap();
        let _ = serde_json::to_writer(file, &self).unwrap_or_else(|_err| {
            eprintln!("ERROR: Could not write to index file");
            exit(1);
        });
    }

    fn from_index_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let tfidf: LDA = serde_json::from_reader(file).expect("Error reading index file");
        tfidf
    }
}

#[cfg(test)]
mod lda_tests {
    use crate::index::index::Index;

    use super::*;

    #[test]
    fn test_handle_token_filter() {
        // Setup LDA
        let topics: Vec<String> = vec![];
        let filter: Vec<String> = vec![
            String::from("the"),
            String::from("a")
        ];
        let mut lda = LDA::new(topics, filter);

        // Handle the token
        let path_buf = PathBuf::from("./");
        let stop_word = String::from("the");
        lda.handle_token(&path_buf, stop_word);

        // Ensure that it is not part of the index
        let doc = lda.document_term_mat.get(&path_buf);
        assert_eq!(doc.is_some(), false);    
    }

    #[test]
    fn test_handle_token_filter_with_doc() {
        // Setup LDA
        let topics: Vec<String> = vec![];
        let filter: Vec<String> = vec![
            String::from("the"),
            String::from("a")
        ];
        let mut lda = LDA::new(topics, filter);

        // Handle the token - this should add the document
        let path_buf = PathBuf::from("./");
        let non_stop_word = String::from("not");
        lda.handle_token(&path_buf, non_stop_word);

        // Handle next token - this should not be added to existing document
        let stop_word = String::from("the");
        lda.handle_token(&path_buf, stop_word.clone());

        // Ensure that it is not part of the index
        let doc = lda.document_term_mat.get(&path_buf);
        assert_eq!(doc.is_some(), true);    

        let freq = doc.unwrap().get(&stop_word.clone()); 
        assert_eq!(freq.is_none(), true);
    }

}
