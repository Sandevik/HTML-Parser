#![allow(dead_code, unused_assignments, unused_mut, unused_variables)]
use crate::utils::Shift;

impl<T> Shift<T> for Vec<T> {
    fn shift(&mut self) -> ()
    where
        T: Clone,
    {
        *self = self[1..].to_vec();
    }
}

#[derive(Debug, Clone)]
pub enum VariableValue {
    String(String),
    Float(f32),
    Integer(i32),
    Bool(bool),
    None,
    Array(Vec<VariableValue>),
    Object(Vec<Variable>),
}

#[derive(Debug, Clone)]
pub struct Variable {
    key: String,
    value: VariableValue,
}

pub struct Json {}

impl Json {
    pub fn parse(string: String) -> Vec<Variable> {
        let mut bytes: Vec<u8> = string.bytes().collect::<Vec<u8>>();
        let mut parsed: Vec<Variable> = Vec::new();
        while bytes.len() > 0 {
            let mut byte = bytes[0];
            if byte == b'{' {
                bytes.shift();
                let object = Self::parse_string_to_object(&mut bytes, None);
                parsed.push(object);
            } else if byte == b'[' {
                println!("parsing array");
                bytes.shift();
                let array = Self::parse_array(&mut bytes);
                parsed.push(Variable {
                    key: "".to_owned(),
                    value: array,
                });
            }
            bytes.shift();
        }
        return parsed;
    }

    fn parse_variables(mut bytes: &mut Vec<u8>) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();
        let mut current_kv_pair: String = String::new();
        let string: String = String::from_utf8(bytes.clone()).unwrap();

        /* TODO: Add compatability for nested objects */
        /* TODO: Remove trailing commas in values? */

        while bytes.len() >= 0 {
            if bytes.len() == 0 {
                variables.push(Self::parse_variable_from_kv(&current_kv_pair));
                break;
            }
            let byte = bytes[0];
            if byte == b'[' {
                while bytes.len() > 0 && bytes[0] != b']' {  
                    // this should become [1,2,34,5,6676]
                    if !bytes[0].is_ascii_whitespace() {
                        current_kv_pair.push(bytes[0] as char);
                    }
                    bytes.shift();
                }
                current_kv_pair.push(']');
            
            } else if (byte == b',' && !String::is_empty(&current_kv_pair)) || byte == b']' {
                variables.push(Self::parse_variable_from_kv(&current_kv_pair));
                current_kv_pair = String::new();
            } else{
                current_kv_pair.push(byte as char);
            }

            bytes.shift();
        }

        return variables
    }

    fn parse_variable_from_kv(str: &str) -> Variable {
        let key = &Self::clean_string(str.split(":").collect::<Vec<&str>>()[0].trim());
        let value_string = str.split(":").collect::<Vec<&str>>()[1];
        let var: Variable = Variable {
            key: key.to_string(),
            value: Self::parse_variable_value(value_string),
        };
        return var;
    }

    fn parse_variable_value(value_string: &str) -> VariableValue {
        let mut bytes: Vec<u8> = value_string.clone().bytes().collect::<Vec<u8>>();
        let value = match value_string.trim() {
            "null" => VariableValue::None,
            "true" => VariableValue::Bool(true),
            "false" => VariableValue::Bool(false),
            str if str.contains("[") => VariableValue::Array(Self::parse_array_values(&mut bytes)),
            str if str.contains("\"") => {
                VariableValue::String(Self::clean_string(value_string.trim()).to_string())
            }
            str if str.contains(".") => VariableValue::Float(
                value_string
                    .trim()
                    .parse::<f32>()
                    .expect(&format!("Could not parse int: {}", value_string)),
            ),
            _ => VariableValue::Integer(
                value_string
                    .trim()
                    .parse::<i32>()
                    .expect(&format!("Could not parse int: {}", value_string)),
            ),
        };
        return value;
    }

    fn parse_array(mut bytes: &mut Vec<u8>) -> VariableValue {
        let mut array: Vec<_> = vec![];
        let mut current_value: String = String::new();
        while bytes.len() > 0 {
            if bytes[0] == b',' {
                array.push(Self::parse_variable_value(&current_value))
            } else {
                current_value.push(bytes[0] as char);
            }
            bytes.shift();
        }
        return VariableValue::Array(array);
    }

    fn parse_array_values(mut bytes: &mut Vec<u8>) -> Vec<VariableValue> {
        let mut values: Vec<VariableValue> = Vec::new();
        let mut current_value: String = String::new();
        while bytes.len() > 0 {
            if bytes[0] != b'[' && bytes[0] != b',' && bytes[0] != b']' {
                current_value.push(bytes[0] as char); 
            }else if bytes[0] == b',' || bytes[0] == b']' {
                values.push(Self::parse_variable_value(&current_value));
                current_value = String::new();
            }
            bytes.shift();
        }
        return values;
    }

    fn parse_string_to_object(mut bytes: &mut Vec<u8>, key: Option<String>) -> Variable {
        let mut variables: Vec<Variable> = Vec::new();
        let mut var: Variable;
        let mut nested_string = String::new();
        while bytes.len() > 0 && bytes[0] != b'}' {
            nested_string.push(bytes[0] as char);
            bytes.shift();
        }
        let obj_vec = Self::parse_variables(&mut nested_string.bytes().collect::<Vec<u8>>());
        return match key {
            Some(key) => Variable {
                key: key,
                value: VariableValue::Object(obj_vec),
            },
            None => Variable {
                key: "".to_string(),
                value: VariableValue::Object(obj_vec),
            },
        };
    }

    fn clean_string(str: &str) -> String {
        return String::from_utf8(
            str.bytes()
                .filter(|char| *char != b'"' || *char != b'\"')
                .collect::<Vec<u8>>(),
        )
        .unwrap();
    }
}
