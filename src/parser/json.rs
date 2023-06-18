#![allow(dead_code, unused_assignments, unused_mut, unused_variables)]
use crate::utils::Shift;
use std::vec;

/* Number */
/* ""a" : 2 " */
/* JsonType::Variable({key: "a", value: Some(2)}) */

/* float */
/* ""b" : 2.17 " */
/* JsonType::Variable({key: "b", value: Some(2.17)}) */

/* String */
/* ""c" : "x" " */
/* JsonType::Variable({key: "c", value: Some("x")}) */

/* Bool */
/* ""d" : true " */
/* JsonType::Variable({key: "d", value: Some(true)}) */

/* Null */
/* "e" : null " */
/* JsonType::Variable({key: "e", value: None}) */

/* Array */
/* ""f" : ["x", "y", "z"] " */
/* JsonType::Variable({key: "f", value: Some(Vec<JsonType)>) */

/* Object */
/* Recursive */

/* JsonTree */
/* ""a" : 2, "b" : "x", "c": [1, 2, 3] " */
/* JsonType::Object(Vec<JsonType>) */

/* Json */
/*
    {
        src: "",
        parsed: JsonType::Object(Vec<JsonType>)),
    }
*/

impl<T> Shift<T> for Vec<T> {
    fn shift(&mut self) -> ()
    where
        T: Clone,
    {
        *self = self[1..].to_vec();
    }
}

#[derive(Debug, Clone)]
enum VariableType {
    String(String),
    Float(f32),
    Integer(i32),
    None
}

#[derive(Debug, Clone)]
pub struct Variable {
    key: String,
    value: VariableType,
}

#[derive(Debug, Clone)]
pub enum JsonType {
    Variable(Variable),
    Object(Vec<JsonType>),
    Array(Vec<JsonType>),
}

pub enum JsonParseType {
    Object,
    Array,
    Variables,
}

pub struct Json {
    parsed: Vec<JsonType>,
}

impl Json {
    
    pub fn parse(string: String) -> Vec<JsonType> {
        let mut bytes: Vec<u8> = string.bytes().collect::<Vec<u8>>();
        let mut parsed: Vec<JsonType> = Vec::new();

        while bytes.len() > 0 {
            let mut byte = bytes[0];
            if byte == b'{' {
                bytes.shift();
                let json_type = Self::parse_string(&mut bytes, JsonParseType::Object);
                parsed.push(json_type);
            } else if byte == b'[' {
                bytes.shift();
                let json_type = Self::parse_string(&mut bytes, JsonParseType::Array);
                parsed.push(json_type);
            }

            bytes.shift();
        }

        return parsed;
    }

    fn parse_string(mut bytes: &mut Vec<u8>, parse_as: JsonParseType) -> JsonType {
        match parse_as {
            JsonParseType::Object => {
                let mut nested_string = String::new();
                while bytes.len() > 0 && bytes[0] != b'}' {
                    nested_string.push(bytes[0] as char);
                    bytes.shift();
                }
                let obj_vec = Self::parse_string(
                    &mut nested_string.bytes().collect::<Vec<u8>>(),
                    JsonParseType::Variables,
                );

                return obj_vec;
            }

            JsonParseType::Array => {
                let vec: Vec<JsonType> = Vec::new();
                let mut nested_string = String::new();
                while bytes.len() > 0 && bytes[0] != b']' {
                    nested_string.push(bytes[0] as char);
                    bytes.shift();
                }
                return JsonType::Array(vec);
            }

            JsonParseType::Variables => {
                let variables = Self::parse_variables(&mut bytes, None);
                return variables;
            }
        }
    }

    fn parse_variables(mut bytes: &mut Vec<u8>, return_as: Option<JsonParseType>) -> JsonType {
        let mut json_type_vec: Vec<JsonType> = Vec::new();

        let string: String = String::from_utf8(bytes.clone()).unwrap();
        let strs: Vec<&str> = string.split(",").collect::<Vec<&str>>();

        for str in strs {
            let key = str.split(":").collect::<Vec<&str>>()[0];
            let value_string = str.split(":").collect::<Vec<&str>>()[1];
            let mut value: VariableType;

            if value_string.trim() == "null" {
                value = VariableType::None
            } else if value_string.contains("\"") {
                value = VariableType::String(value_string.to_string());
            } else if value_string.contains(".") {
                value = VariableType::Float(
                    value_string
                        .trim()
                        .parse::<f32>()
                        .expect(&format!("Could not parse {}", value_string)),
                );
            } else {
                value = VariableType::Integer(
                    value_string
                        .trim()
                        .parse::<i32>()
                        .expect(&format!("Could not parse {}", value_string)),
                )
            }

            let var: Variable = Variable {
                key: key.to_string(),
                value: value,
            };

            json_type_vec.push(JsonType::Variable(var));
        }

        return match return_as {
            Some(JsonParseType::Variables) => JsonType::Array(json_type_vec),
            Some(JsonParseType::Array) => JsonType::Array(json_type_vec),
            Some(JsonParseType::Object) | None => JsonType::Object(json_type_vec),
        };
    }
}
