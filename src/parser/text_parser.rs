use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn parse_txt_file(file: &mut BufReader<File>) -> io::Result<String> {
    let mut buf = String::new();
    let _file = file.read_to_string(&mut buf)?;
    Ok(buf)
}

