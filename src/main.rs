//use std::fs;
mod parser;
mod utils;

use parser::json::Json;

fn main() {

    //let html = String::from("<!DOCTYPE html><html class=\"\" [sdadsa] sadads-fsdaff >content abc 123</html>"); //fs::read_to_string("index.html").expect("File not found");

    println!("{:#?}", Json::parse("{ \"a\" : 3.14, \"b\" : null, \"d\": [1, 2, 3, 4, 5 ,6, 7]}"));

    //HTMLParser::extract_tag_name(&"<html class=\"\" class2 assafs [asda]asdads]>".to_string());

}
