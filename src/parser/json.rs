#![allow(dead_code, unused_mut, unused_variables, unreachable_patterns)]
use std::{collections::HashMap};

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub enum VariableTypedValue {
    Object(HashMap<String, VariableTypedValue>),
    Array(Vec<VariableTypedValue>),
    Value(Value)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident(String),
    Colon,
    Comma,
    OpenCurly,
    CloseCurly,
    OpenBrack,
    CloseBrack
}

#[derive(Debug)]
pub struct Consumer {
    pub pos: usize,
    pub ch: char,
    pub size: usize,
    vec: Vec<u8>
}
impl Consumer {
    pub fn new(src: &str) -> Consumer {
        return Consumer {
            vec: src.bytes().collect::<Vec<u8>>(),
            pos: 0,
            ch: src.bytes().collect::<Vec<u8>>()[0] as char,
            size: src.bytes().len(),
        }
    }

    pub fn eat(&mut self) -> char {
        if  self.pos + 1 < self.size {
            self.pos += 1;
            self.ch = self.vec[self.pos] as char;
        } else {
            self.pos = self.size;
        }
        return self.ch; 
    }

    pub fn peek(&self) -> Result<char, String> {
        if self.pos + 1 < self.size {
            return Ok(self.vec[self.pos + 1] as char)
        }else{
            return Err("EOF / EOL".to_string())
        }
    }
}

pub struct Tokenizer {}
impl Tokenizer {
    pub fn tokenize(string: &str) -> Vec<TokenType>{
        let mut token_types: Vec<TokenType> = Vec::new();
        let mut value: String = String::new();
        let mut consumer: Consumer = Consumer::new(string);
        while consumer.pos < consumer.size{
            match consumer.ch {
                ',' => {
                    if !value.is_empty() {
                        token_types.push(TokenType::Ident(value.trim().to_string()));
                        value = String::new();
                    }
                    token_types.push(TokenType::Comma);
                    },
                ':' => {
                    if !value.is_empty() {
                        token_types.push(TokenType::Ident(value.trim().to_string()));
                        value = String::new();
                    }
                    token_types.push(TokenType::Colon)
                    },
                '{' => {
                    token_types.push(TokenType::OpenCurly)
                },
                '}' => {
                    if !value.is_empty() {
                        token_types.push(TokenType::Ident(value.trim().to_string()));
                        value = String::new();
                    }
                    token_types.push(TokenType::CloseCurly)
                },
                '[' => {
                    token_types.push(TokenType::OpenBrack)
                },
                ']' => {
                    if !value.is_empty() {
                        token_types.push(TokenType::Ident(value.trim().to_string()));
                        value = String::new();
                    }
                    token_types.push(TokenType::CloseBrack)
                },
                
                ch => value.push(ch),
            }
            consumer.eat();
        }
        if !value.is_empty() {
            token_types.push(TokenType::Ident(value));
        }
        return token_types
    }
}

pub struct Parser {}
impl Parser {

    pub fn variablize(string: &str) -> Vec<VariableTypedValue> {
        let mut variables: Vec<VariableTypedValue> = Vec::new(); 
        let mut tokens: Vec<TokenType> = Tokenizer::tokenize(string);
        while tokens.len() > 0 {
            let token = &tokens[0];
            match token {
                TokenType::OpenCurly => {},//println!("{{ parse obj }}"),   /* variables.push(Self::parse_obj(&mut tokens)), */
                TokenType::CloseCurly => {},//println!("{{ obj ended }}"),
                TokenType::OpenBrack => variables.push(Self::parse_arr(&mut tokens)),

                TokenType::Ident(value) => {}//println!("ident: {:?}", Self::parse_value_from_string(TokenType::Ident(value.clone())).unwrap()),
                TokenType::Colon => {},//println!("previous value was key, next is value"),
                TokenType::Comma => {},//println!("new variable"),
                
                token => {},//println!("Token \"{:?}\" has not been matched yet", token)
            }
            if tokens.len() > 0 {
                tokens = tokens[1..].to_vec();
            } 
        }
        println!("{:?}", variables);
        return variables;
    }

    fn parse_arr(tokens: &mut Vec<TokenType>) -> VariableTypedValue {
        let mut values: Vec<VariableTypedValue> = Vec::<VariableTypedValue>::new();
        //consume the "[" to prevent infinite loop
        *tokens = tokens[1..].to_vec();
        
        while tokens.len() > 0 && tokens[0] != TokenType::CloseBrack {
            let token: &TokenType = &tokens[0];
            match token {
                TokenType::OpenBrack => {
                    values.push(Self::parse_arr(tokens));
                },
                TokenType::Ident(val) => values.push(VariableTypedValue::Value(Self::parse_value_from_string(token.clone()).unwrap())),
                _ => {}
            }
            if tokens.len() > 0 {
                *tokens = tokens[1..].to_vec();
            } 
        }
        //consume the "]" to prevent adding it as a variable
        if tokens.len() > 0 {
            *tokens = tokens[1..].to_vec();
        } 
        return VariableTypedValue::Array(values);
    }

    fn parse_obj(tokens: &mut Vec<TokenType>) -> VariableTypedValue {
        let mut hashmap: HashMap<String, VariableTypedValue> = HashMap::new();
        let mut key: String = String::new();
        let mut value: VariableTypedValue = VariableTypedValue::Value(Value::Null);

        //consume "{" to prevent infinate loop
        *tokens = tokens[1..].to_vec();
        while tokens.len() > 0 && tokens[0] != TokenType::CloseCurly {
            let token = &tokens[0];

            if *token != TokenType::OpenCurly {
                if key.is_empty(){
                    match token {
                        TokenType::Ident(val) => key = val.clone(),
                        _ => {}
                    }
                } else {
                    match token {
                        TokenType::Ident(val) => value = VariableTypedValue::Value(Self::parse_value_from_string(token.clone()).unwrap()),
                        _ => {}
                    }
                    hashmap.insert(key, value);
                    key = String::new();
                    value = VariableTypedValue::Value(Value::Null);
                }
            }
    
            *tokens = tokens[1..].to_vec();
        }
        //consume the "}" to prevent adding it as a variable
        *tokens = tokens[1..].to_vec();


        VariableTypedValue::Object(hashmap)
    }

    fn parse_value_from_string(token: TokenType) -> Result<Value, String> {
        match token {
            TokenType::Ident(value) => {
                match value.as_str() {
                    "null" => Ok(Value::Null),
                    "true" => Ok(Value::True),
                    "false" => Ok(Value::False),
                    str if str.contains("\"") || str.bytes().filter(|x| *x == b'.').collect::<Vec<u8>>().len() > 1 => Ok(Value::String(Self::clean_string(str))),
                    str if str.contains(".") => Ok(Value::Float(str.parse::<f32>().unwrap())),
                    str => Ok(Value::Int(str.parse::<i32>().unwrap())),
                    str => {
                        Err(format!("Could not parse value: \"{}\"", str))
                    }
                }
            },
            t => Err(format!("Token value cannot be parsed, {:?}", t))
        }
    }

    fn clean_string(str: &str) -> String {
        str.chars().filter(|ch| *ch != '\"').collect::<String>()
    }

}


