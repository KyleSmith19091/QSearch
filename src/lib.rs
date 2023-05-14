use std::{fs::{read_dir, File},io::{self, BufReader}, collections::HashMap, path::PathBuf};

pub mod lexer;
pub mod parser;
pub mod index;

use index::index::Index;
use parser::text_parser::parse_txt_file;
use lexer::lexer::Lexer;

pub type ParserFuncMap = HashMap<String, Box<dyn Fn(&mut BufReader<File>) -> io::Result<String>>>;

// Parse file to a string representation
pub fn parse_file(parser_func_map: &ParserFuncMap, path: &PathBuf) -> io::Result<String> {

    // Get file extension
    let file_extension_opt = path.extension();

    // Check if path ends with a file with a valid extension
     match file_extension_opt {
         Some(file_extension) => {
             // Open file at path
            if let Ok(file) = File::open(path.to_str().unwrap()) {
                // Create Buffered reader from file
                let mut reader = BufReader::new(file);

                // Attempt matching file extension to parser otherwise parse as text
                match parser_func_map.get(file_extension.to_str().unwrap()) {
                    Some(func) => { 
                        return func(&mut reader); 
                    }
                    None => { 
                        return parse_txt_file(&mut reader); 
                    }
                }
            } else {
                let error_msg = format!("Could not open file {:?}", path);
                return Err(io::Error::new(io::ErrorKind::Other, error_msg));
            }
        }
        None => {
            let error_msg = format!("File extension is missing for file {:?}", path);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, error_msg))
        }
     }
}

pub fn index_dir(path: &str, parser_func_map: &ParserFuncMap, relevance_struct: &mut impl Index) -> io::Result<()>{
    read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .for_each(|res| {
            // Get the path
            let res = res.unwrap();

            if res.is_dir() { // Recursively index directory
                index_dir(res.to_str().unwrap(), &parser_func_map, relevance_struct).unwrap_or_else(|_err|{
                    println!("Error Indexing Directory... {res:?}");
                });
            } else { // Otherwise index file
                // Parse the file
                if let Ok(text) = parse_file(&parser_func_map, &res) {
                    // Convert string to char array
                    let text = text.chars().collect::<Vec<_>>();

                    // Tokenise the content of the file
                    for token in Lexer::new(&text) {
                        // Give token to index struct to handle
                        relevance_struct.handle_token(&res, token.into_iter().map(|x| x.to_ascii_uppercase()).collect::<String>());
                    }

                    println!("Done Indexing: {res:?}");
                } else {
                    println!("Error Indexing File skipping... {res:?}");
                }
            }
        });

    Ok(())
}

pub fn query(query: String, relevance_struct: &impl Index) -> Vec<(&PathBuf, f64)> {
    let mut terms: Vec<String> = vec![];

    // Construct lexer from query
    let query = &query.chars().map(|x| x.to_ascii_uppercase()).collect::<Vec<_>>();
    let lexer = Lexer::from_vec(query);

    // Construct query terms from tokeniser
    for token in lexer {
        let term = token.to_owned().into_iter().collect::<String>();
        terms.push(term);
    }

    // Query the index for results
    return relevance_struct.query(&terms);
}

