use std::vec;
use crate::utils::Shift;

/* Number */
/* "{ "a" : 2 }" */
/* JsonType::Variable({key: "a", value: Some(2)}) */

/* float */
/* "{ "b" : 2.17 }" */
/* JsonType::Variable({key: "b", value: Some(2.17)}) */

/* String */
/* "{ "c" : "x" }" */
/* JsonType::Variable({key: "c", value: Some("x")}) */

/* Bool */
/* "{ "d" : true }" */
/* JsonType::Variable({key: "d", value: Some(true)}) */

/* Null */
/* "{ "e" : null }" */
/* JsonType::Variable({key: "e", value: None}) */

/* Array */
/* "{ "f" : ["x", "y", "z"] }" */
/* JsonType::Variable({key: "f", value: Some(Vec<JsonType)>) */

/* Object */
/* Recursive */

/* JsonTree */
/* "{ "a" : 2, "b" : "x", "c": [1, 2, 3]}" */
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
pub struct Variable<T> {
    key: String,
    value: T,
}

#[derive(Debug, Clone)]
pub enum JsonType<T> {
    Variable(Variable<Option<T>>),
    Object(Vec<JsonType<T>>),
}

pub enum JsonParseType {
    String,
    Object,
    Array,
}

pub struct Json<T> {
    src: String,
    parsed: Option<Vec<JsonType<T>>>,
}

impl<T> Json<T> {
    pub fn parse(string: String) {
        let mut vectorized: Vec<String> = Self::vectorize_string(string);
    }

    fn vectorize_string(string: String) -> Vec<String> {
        let mut bytes: Vec<u8> = string.bytes().collect::<Vec<u8>>();
        let mut vectorized: Vec<String> = Vec::new();

        while bytes.len() > 0 {
            let mut byte = bytes[0];

           
        }

        vectorized
    }

    fn parse_string(mut bytes: &mut Vec<u8>, parse_as: JsonParseType) {
        todo!("Variableize thing");
    }
}
