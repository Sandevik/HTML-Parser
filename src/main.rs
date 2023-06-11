use std::fs;

mod htmlparser;
use htmlparser::HTMLParser;

fn main() {

    let html = fs::read_to_string("index.html").expect("File not found");



    println!("{:#?}", HTMLParser::parse(&html));
}
