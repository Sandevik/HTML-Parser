use std::vec;

use crate::utils::Shift;

#[derive(Debug)]
pub enum JsonType{
    String(String),
    Number(String),
    Object(Vec<JsonType>),
    Array(Vec<JsonType>),
    Null,
    Boolean(bool)
}



pub struct Json {
    string: String,
    json_vec: Option<Vec<JsonType>>

} 

impl Json {
    pub fn parse(string: String){
        let mut vectorized: Vec<String> = Self::vectorize_string(string);
    }

    fn vectorize_string(string: String) -> Vec<String> {
        let mut bytes: Vec<u8> = string.bytes().collect::<Vec<u8>>();
        let mut vectorized: Vec<String> = Vec::new();


        while bytes.len() > 0 {
            let mut byte = bytes[0];
            
            if byte == b'[' {
                println!("parse as array");
                let parsed_arr: Vec<JsonType> = Self::parse_string(&mut bytes, JsonParseType::Array);

                println!("parsed arr: {:?}", JsonType::Array(parsed_arr));

            }else if byte == b'{' {
                println!("parse as object");
                let parsed_obj: Vec<JsonType> = Self::parse_string(&mut bytes, JsonParseType::Object);
                
                println!("parsed arr: {:?}", JsonType::Object(parsed_obj));

            }else{
                println!("{:?}", Self::parse_string(&mut bytes, JsonParseType::String));
            }

            
        } 

        todo!();
    }

    

    fn parse_string(mut bytes: &mut Vec<u8>, parse_as: JsonParseType) -> Vec<JsonType> {
        bytes.shift();
        match parse_as {
            JsonParseType::String => {
                let mut ident_vec: Vec<JsonType> = Vec::new();
                let mut ident: String = String::new();
                while bytes.len() > 0 {
                    let byte = bytes[0];
                    if byte == b':' || byte == b',' {
                        ident_vec.push(JsonType::String(ident.trim().to_string()));
                    }else if byte == b'{' {
                        Self::parse_string(&mut bytes, JsonParseType::Object);
                    } else {
                        ident.push(byte as char);
                    }
                
                    bytes.shift();
                }
                return ident_vec;
            },
            JsonParseType::Array => {
                let mut arr_vec: Vec<JsonType> = Vec::new();
                let mut arr_item = String::new();

                while bytes.len() > 0 && bytes[0] != b']' {
                    if bytes[0] == b',' {
                        arr_vec.push(JsonType::String(arr_item));
                        arr_item = String::new();
                    }else{
                        arr_item.push(bytes[0] as char)
                    }
                    bytes.shift();
                }

                return arr_vec;
            },
            JsonParseType::Object => {
                let obj_vec: Vec<JsonType> = Vec::new();

                return obj_vec;
            },
            _ => panic!("This type is not supported")
        }
    }

    
}

impl<T> Shift<T> for Vec<T>{
    fn shift(&mut self) -> () where T: Clone {
        *self = self[1..].to_vec();
    }
}

pub enum JsonParseType {
    String,
    Object,
    Array,
}