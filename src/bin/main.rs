extern crate qsearch;

use qsearch::index::index::Index;
use qsearch::index::tfidf::*;
use qsearch::index_dir;
use qsearch::parser::xml_parser::*;
use qsearch::parser::text_parser::*;
use qsearch::parser::html_parser::*;

fn index(dir: &str, path: &str) -> bool {
    let mut tfidf = TFIDF::new();

    let mut parser_func_map = qsearch::ParserFuncMap::new();
    parser_func_map.insert(String::from("xml"), Box::from(parse_xml_file));
    parser_func_map.insert(String::from("txt"), Box::from(parse_txt_file));
    parser_func_map.insert(String::from("html"), Box::from(parse_html_file));
    if let Ok(_result) = index_dir(dir, &parser_func_map, &mut tfidf) {
        tfidf.to_index_file(path); 
        return true;
    } else {
        return false;
    }
}

fn read_index(path: &str) {
    // Create Index
    let tfidf = TFIDF::from_index_file(path);
    let result = qsearch::query(String::from("how to"), &tfidf);

    for (path,rank) in result.iter().take(5) {
        println!("{path} => {rank}", path = path.display());
    }
}

fn main() {
    const INDEX_DIR: &str = "./website/content/en/docs/";
    const INDEX_FILE: &str = "index-web.json";

    read_index(INDEX_FILE);
}
