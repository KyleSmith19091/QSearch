use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use std::fs::File;
use serde::{Serialize, Deserialize};

use crate::index::index;

#[derive(Serialize, Deserialize)]
pub struct TFIDF {
    global_index: HashMap<PathBuf, HashMap<String, u64>>,
    terms_per_document: HashMap<PathBuf, u64>,
}

impl TFIDF {
    pub fn new() -> Self {
        Self { 
            global_index: HashMap::<PathBuf, HashMap::<String,u64>>::new(), 
            terms_per_document: HashMap::<PathBuf, u64>::new(),
        }
    }

    pub fn get_file_tokens(&self, path: &PathBuf) -> Option<Vec::<(&String,&u64)>> {
        if let Some(tf) = self.global_index.get(path) {
            let mut stats = tf.iter().collect::<Vec<_>>();
            stats.sort_by_key(|(_,f)| *f);
            stats.reverse();
            return Some(stats);
        }
        None
    }

    pub fn get_term_frequency(&self, term: &str, path: &PathBuf) -> f64 {
        if let Some(tf) = self.global_index.get(path) {
            if let Some(freq) = tf.get(term) {
                return (
                    (*freq as f64) / (*self.terms_per_document.get(path).unwrap()
                ) as f64) as f64;
            } else {
                return 0.0;
            }
        }
        0.0
    }

    pub fn get_number_docs_with_term(&self, term: &str) -> Vec<&PathBuf> {
        let mut doc_paths: Vec<&PathBuf> = vec![];
        for (path, tf) in self.global_index.iter() {
            if let Some(_freq) = tf.get(term) {
                doc_paths.push(path);        
            }
        }
        return doc_paths;
    }

    pub fn get_inverse_document_freq(&self, term: &str) -> f64 {
        let docs = self.get_number_docs_with_term(term);
        
        if docs.len() == 0 {
            return 0.0;
        }

        let inverse_docs_ratio = (self.terms_per_document.keys().len() / docs.len()) as f64;
        return f64::log10(inverse_docs_ratio);
    }

    pub fn get_tfidf(&self, query_words: &Vec<String>) -> Vec<(&PathBuf, f64)> {
        let mut paths: Vec<(&PathBuf, f64)> = vec![];

        for (path, _tf) in self.global_index.iter() {
            let mut rank = 0f64;
            for term in query_words {
                let tf = self.get_term_frequency(term, &path);
                let idf = self.get_inverse_document_freq(term);
                rank += tf * idf;
            }
            paths.push((path, rank));
        }
        paths.sort_by(|(_, rank_a), (_, rank_b)|{
            rank_b.partial_cmp(rank_a).unwrap()
        });
        paths
    }
}

impl index::Index for TFIDF {
    fn query(&self, query_words: &Vec<String>) -> Vec<(&PathBuf, f64)> {
        return self.get_tfidf(query_words);
    }

    fn handle_token(&mut self, path: &PathBuf, token: String) {
        if let Some(tf) = self.global_index.get_mut(path) {
            if let Some(freq) = tf.get_mut(&token) {
                *freq += 1;
                *self.terms_per_document.get_mut(path).unwrap() += 1;
            } else {
                tf.insert(token, 1);                
                *self.terms_per_document.get_mut(path).unwrap() += 1;
            }
        } else {
            self.global_index.insert(
                path.to_owned(), 
                HashMap::<String, u64>::from([(token,1)])
            );
            self.terms_per_document.insert(path.to_owned(), 1);
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
        let tfidf: TFIDF = serde_json::from_reader(file).expect("Error reading index file");
        tfidf
    }
}

