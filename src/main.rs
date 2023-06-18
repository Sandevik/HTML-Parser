use std::fs;
mod parser;
mod utils;

use parser::json::Json;

fn main() {

    //let html = String::from("<!DOCTYPE html><html class=\"\" [sdadsa] sadads-fsdaff >content abc 123</html>"); //fs::read_to_string("index.html").expect("File not found");

    println!("{:#?}", Json::parse("\"test\": \"hej\", { \"x\" : 2, \"y\": 3, \"z\": \"4\", \"a\" : 3.14, \"b\" : null, \"c\" : true, \"d\": false }".to_string()));

    //HTMLParser::extract_tag_name(&"<html class=\"\" class2 assafs [asda]asdads]>".to_string());

}
