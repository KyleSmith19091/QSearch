use std::{fs::File, io::BufReader};
use std::io;
use xml::reader::{EventReader,XmlEvent};

pub fn parse_xml_file(file: &mut BufReader<File>) -> io::Result<String> {
    let reader = EventReader::new(file);
    let mut content = String::new();

    for event in reader.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
            content.push_str(" ");
        }
    }

    Ok(content)
}
