use std::fs::File;
use std::io::{BufReader, Read};
use std::io;

pub fn parse_html_file(file: &mut BufReader<File>) -> io::Result<String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let dom = tl::parse(&buf, tl::ParserOptions::default()).unwrap();

    let mut content = String::new();
    dom.nodes()
        .iter()
        .for_each(|node|{
            let inner_text = node.inner_text(dom.parser()).to_string();
            if inner_text.len() > 1 {
                content.push_str(&inner_text);
                content.push_str(" ");
            }
        });

    return Ok(content);
}
