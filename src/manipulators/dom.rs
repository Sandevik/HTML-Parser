#![allow(dead_code, unused_variables, unused_assignments, unused_mut)]

use std::{collections::HashMap, rc::Rc};
use crate::utils::clean_string;

#[derive(Debug)]
pub struct Consumer {
    buf: Rc<[u8]>,
    pos: usize,
    size: usize,
    ch: char,
}
impl Consumer {
    pub fn from(str: &str) -> Consumer {
        Consumer {
            buf: str.bytes().collect::<Rc<[u8]>>(),
            pos: 0,
            size: str.bytes().len(),
            ch: str.chars().nth(0).unwrap(),
        }
    }

    pub fn eat(&mut self) -> char {
        if self.pos + 1 < self.size {
            self.pos += 1;
            self.ch = self.buf[self.pos] as char;
        } else {
            self.pos = self.size;
        }
        return self.ch;
    }
    pub fn peek(&self) -> char {
        if self.pos + 1 < self.size {
            return self.buf[self.pos + 1] as char;
        } else {
            return 0 as char;
        }
    }
}



#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Open,
    Close,
    SelfClosing,
    None,
    Comment,
    Content,
    PHP
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    tag: Tag,
    raw: String,
    tag_type: TokenType,
}



#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Doctype,
    XML,

    Html,
    Head,
    Body,
    Meta,
    Title,

    Div,
    Span,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,

    Ul,
    Ol,
    Li,

    Img,

    PHP,
    
    None,
    Unknown,
    Tag(String),
    Root,
}

pub type Attributes = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct Element {
    tag: Tag,
    content: Option<String>,
    attributes: Option<Attributes>,
    children: Option<Vec<Element>>,
}
impl Element {
    pub fn default() -> Element {
        Element {
            tag: Tag::Unknown,
            content: None,
            attributes: None,
            children: None
        }
    }

    pub fn new(tag: Tag, content: Option<String>, attributes: Option<Attributes>, children: Option<Vec<Element>>) -> Element {
        Element {
            tag: tag,
            content: content,
            attributes: attributes,
            children: children,
        }
    }
}

pub struct DOM {}
impl DOM {
    pub fn tokenize(mut consumer: Consumer) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut token: Token = Token { tag: Tag::None, raw: "".to_string(), tag_type: TokenType::None };


        let mut token_type: TokenType = TokenType::None;
        let mut ident: String = String::new();
        while consumer.pos < consumer.size {
            match consumer.ch {
                '<' => {
                    if ident.len() > 0 {
                        tokens.push(Token {tag: Tag::None, raw: ident, tag_type: TokenType::Content});
                        ident = String::new();
                    }
                    if consumer.peek() == '/' {
                        //closing tag
                        token_type = TokenType::Close;
                        ident.push(consumer.ch);
                    } else if consumer.peek() == '!' {
                        if consumer.buf[consumer.pos + 2] == b'D' {
                            //Doctype
                            token_type = TokenType::SelfClosing;
                        } else {
                            // comment
                            token_type = TokenType::Comment;
                        }
                        ident.push(consumer.ch);
                    } else if consumer.peek() == '?' {
                        //xml declatation
                        /* TODO: Add functionality to parse PHP */
                        if consumer.buf[consumer.pos + 2] != b'p'
                            && consumer.buf[consumer.pos + 2] != b'='
                        {
                            token_type = TokenType::SelfClosing
                        } else {
                            token_type = TokenType::PHP
                        }
                        ident.push(consumer.ch);
                    } else {
                        // opening tag
                        token_type = TokenType::Open;
                        ident.push(consumer.ch);
                    }
                }
                '/' => {
                    if consumer.peek() == '>' {
                        //self closing
                        token_type = TokenType::SelfClosing;
                        ident.push(consumer.ch);
                    } else {
                        ident.push(consumer.ch);
                    }
                }
                '>' => {
                    ident.push('>');
                
                    let copy_ident = ident.clone();
                    let token: Token = match token_type {
                        TokenType::SelfClosing => Token {tag: Self::parse_tag(&copy_ident), raw: copy_ident, tag_type: TokenType::SelfClosing},
                        TokenType::Open => Token {tag: Self::parse_tag(&copy_ident), raw: copy_ident, tag_type: TokenType::Open},
                        TokenType::Close => Token {tag: Self::parse_tag(&copy_ident), raw: copy_ident, tag_type: TokenType::Close},
                        TokenType::Comment => Token {tag: Self::parse_tag(&copy_ident), raw: copy_ident, tag_type: TokenType::Comment},
                        TokenType::None => Token {tag: Tag::None, raw: copy_ident, tag_type: TokenType::None},
                        TokenType::PHP => Token {tag: Self::parse_tag(&copy_ident), raw: copy_ident, tag_type: TokenType::PHP},
                        TokenType::Content => Token {tag: Tag::None, raw: copy_ident, tag_type: TokenType::Content }
                    };
                    tokens.push(token);
                    ident = String::new();
                }
                _ => ident.push(consumer.ch),
            }
            consumer.eat();
        }
        return tokens;
    }

    pub fn parse(consumer: Consumer) -> Element {
        let mut root_element: Element = Element::default();
        let mut tokens = Self::tokenize(consumer);
        root_element.children = Self::parse_elements(&mut tokens);
        root_element.tag = Tag::Root;
        return root_element;
    }

    fn convert_tag(str: &str) -> Vec<String> {
        /* Will convert a string like "<html lang="en">" to ["html", "lang=\"en\""] */
        let mut str = str.strip_prefix(| p | p == '<').unwrap_or(str).strip_suffix(|p| p == '>').unwrap_or(str).to_string();
        let mut attrs: Vec<String> = Vec::new();
        let mut attribute = String::new();
        let mut in_quote: bool = false;

        while str.len() > 0 {
            match str.chars().nth(0).unwrap() {
                '\"' => in_quote = !in_quote,
                ' ' => {
                    if in_quote {
                        attribute.push(' ');
                    } else {
                        attrs.push(attribute);
                        attribute = String::new();
                    }
                }
                '/' => {
                    if in_quote {
                        attribute.push('/');
                    }
                }
                ch => attribute.push(ch),
            }
            str = String::from_utf8(str.bytes().collect::<Vec<u8>>()[1..].to_vec()).unwrap();
        }
        if !attribute.is_empty() {
            attrs.push(attribute);
        }
        return attrs
    }

    fn parse_attributes(str: &str) -> Option<Attributes> {
        let attrs = Self::convert_tag(str)[1..].to_vec();
        if attrs.len() < 1 {
            return None;
        } else {
            let mut attr: Attributes = Attributes::new();
            for mut str in attrs.clone() {
                let mut key = String::new();
                let mut val = String::new();
                let mut is_key: bool = true;
                let attrs = str.split("=").collect::<Vec<&str>>();
                key = attrs[0].to_string();
                if attrs.len() > 1 {
                    val = clean_string(attrs[1]);
                }
                if key != "/" {
                    attr.insert(key, val);
                }
                key = String::new();
                val = String::new();
            }
            return Some(attr);
        }
    }

    fn parse_tag(str: &str) -> Tag {
        let tag: &String = &Self::convert_tag(&str)[0];
        match tag.as_str() {
            "!DOCTYPE" => Tag::Doctype,
            "?xml" => Tag::XML,
            "html" => Tag::Html,
            "head" => Tag::Head,
            "title" => Tag::Title,
            "meta" => Tag::Meta,
            "body" => Tag::Body,
            "div" => Tag::Div,
            "span" => Tag::Span,
            "img" => Tag::Img,
            "h1" => Tag::H1,
            "h2" => Tag::H2,
            "h3" => Tag::H3,
            "h4" => Tag::H4,
            "h5" => Tag::H5,
            "h6" => Tag::H6,
            "p" => Tag::P,
            "li" => Tag::Li,
            "ol" => Tag::Ol,
            "ul" => Tag::Ul,
                /* TODO: Add more! */

            "?php" => Tag::PHP,

            str => todo!("Lägg till fler taggar: {}", str)
        }
    }

    fn parse_php_tag(str: &str) -> Element {

        return Element::default();
    }

    fn parse_elements(mut tokens: &mut Vec<Token>) -> Option<Vec<Element>> {
        let mut elements: Vec<Element> = Vec::<Element>::new();
        let mut element: Element = Element::default();

        while tokens.len() > 0 {
            match tokens[0].tag_type.clone() {

                TokenType::Open => {
                    *tokens = tokens[1..].to_vec();
                    element.tag = Self::parse_tag(&tokens[0].raw);
                    element.attributes = Self::parse_attributes(&tokens[0].raw);
                    element.children = Self::parse_elements(tokens)
                },

                TokenType::Content => {
                    if element.content.is_some() {
                        let mut content = element.content.unwrap();
                        content.push_str(&tokens[0].raw);
                        element.content = Some(content);
                    } else {
                        element.content = Some(tokens[0].raw.to_string());
                    }
                },




                TokenType::Close => {
                    elements.push(element);
                    element = Element::default();
                }, 

                TokenType::SelfClosing => {
                    element.content = None;
                    element.children = None;
                    element.attributes = Self::parse_attributes(&tokens[0].raw);
                    element.tag = Self::parse_tag(&tokens[0].raw);
                    elements.push(element);
                    element = Element::default();
                },

                TokenType::PHP => todo!(),

                _ => {},
            }

            if tokens.len() > 0 {
                *tokens = tokens[1..].to_vec();
            }
        }
        if elements.len() > 0 {
            return Some(elements)
        }else{
            return None;
        }
    }

    
}
