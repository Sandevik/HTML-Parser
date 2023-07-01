//use std::fs;
mod parser;
mod utils;

use parser::json::Json;

fn main() {

    //let html = String::from("<!DOCTYPE html><html class=\"\" [sdadsa] sadads-fsdaff >content abc 123</html>"); //fs::read_to_string("index.html").expect("File not found");

    println!("{:#?}", Json::parse("{\"firstName\" : \"John\",\"lastName\" : \"Doe\",\"age\" : 23,\"residency\" : {\"address\" : \"One Way 21\",\"zip\" : 123567,\"city\" : \"Big City\"},\"pets\" : [{\"animal\" : \"cat\", \"age\" : 2, \"name\" : \"Tom\"}, {\"animal\" : \"mouse\", \"age\" : 1, \"name\" : \"Jerry\"}],\"lastCoordinates\" : [[\"lat 84,45369\", \"long 12.5467\"], [\"lat 55.255657\", \"long 67.35677\"]]}"));

    //HTMLParser::extract_tag_name(&"<html class=\"\" class2 assafs [asda]asdads]>".to_string());

}
