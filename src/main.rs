use std::fs;
mod parser;
use parser::HTMLParser;

fn main() {

    let html = fs::read_to_string("index.html").expect("File not found");

    let mut parser: HTMLParser = HTMLParser::new();
    parser.parse(&html);

}
