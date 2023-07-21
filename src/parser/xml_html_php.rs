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

#[derive(Debug, Clone)]
pub enum Token {
    Doctype(String),
    XML(String),
    PHP(String),
    StartTag(String),
    EndTag(String),
    SelfClosing(String),
    Comment(String),
    Content(String),
    EOF,
}
enum TokenType {
    Doctype,
    StartTag,
    EndTag,
    SelfClosing,
    Comment,
    EOF,
    Unknown,
    XML,
    PHP,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
    
    
    Unknown,
    Root,
}

pub type Attributes = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct Element {
    tag: Tag,
    content: Option<String>,
    attr: Option<Attributes>,
    children: Option<Vec<Element>>,
}
impl Element {
    pub fn default() -> Element {
        Element {
            tag: Tag::Unknown,
            content: None,
            attr: None,
            children: None
        }
    }

    pub fn new(tag: Tag, content: Option<String>, attr: Option<Attributes>, children: Option<Vec<Element>>) -> Element {
        Element {
            tag: tag,
            content: content,
            attr: attr,
            children: children,
        }
    }
}

pub struct Parser {}
impl Parser {
    pub fn tokenize(mut consumer: Consumer) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut token_type: TokenType = TokenType::Unknown;
        let mut ident: String = String::new();
        while consumer.pos < consumer.size {
            match consumer.ch {
                '<' => {
                    if ident.len() > 0 {
                        tokens.push(Token::Content(ident));
                        ident = String::new();
                    }
                    if consumer.peek() == '/' {
                        //closing tag
                        token_type = TokenType::EndTag;
                        ident.push(consumer.ch);
                    } else if consumer.peek() == '!' {
                        if consumer.buf[consumer.pos + 2] == b'D' {
                            //Doctype
                            token_type = TokenType::Doctype;
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
                            token_type = TokenType::XML
                        } else {
                            token_type = TokenType::PHP
                        }
                        ident.push(consumer.ch);
                    } else {
                        // opening tag
                        token_type = TokenType::StartTag;
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
                        TokenType::Doctype => Token::Doctype(copy_ident),
                        TokenType::XML => Token::XML(copy_ident),
                        TokenType::StartTag => Token::StartTag(copy_ident),
                        TokenType::EndTag => Token::EndTag(copy_ident),
                        TokenType::SelfClosing => Token::SelfClosing(copy_ident),
                        TokenType::Comment => Token::Comment(copy_ident),
                        TokenType::EOF => Token::EOF,
                        TokenType::Unknown => Token::Content(copy_ident),
                        TokenType::PHP => Token::PHP(copy_ident),
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
        let mut tokens: Vec<Token> = Self::tokenize(consumer);
        root_element.children = Self::parse_elements(&mut tokens, None);
        root_element.tag = Tag::Root;
        return root_element;
    }

    fn parse_attributes(str: &str) -> Option<Attributes> {
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

        if attrs.len() < 1 {
            return None;
        } else {
            let mut attr: Attributes = Attributes::new();
            for mut str in attrs[1..].to_vec().clone() {
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
        let str = str.strip_prefix(|p| p == '<').unwrap_or(str);

        /* TODO: Redo match with regex? */

        match str.split(' ').collect::<Vec<&str>>()[0] {
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
                /* TODO: Add more! */

            "<?php" => Tag::PHP,

            _ => Tag::Unknown
        }
    }

    fn parse_php_tag(str: &str) -> Element {

        return Element::default();
    }

    fn parse_elements(tokens: &mut Vec<Token>, tag: Option<Tag>) -> Option<Vec<Element>> {

        let mut elements: Vec<Element> = Vec::<Element>::new();

        // while tokens
        // if token type = Open
        //  parse token details
        //  eat token
        //  parse_elements() while token != EndTag && tag != prev open tag
        //  add tokens to element
        // return element


        let mut tag: Tag = tag.unwrap_or(Tag::Unknown);
        let mut next_tag: Tag = Tag::Unknown;
        let mut element: Element = Element::default();


        
        while tokens.len() > 0 {
            match &tokens[0] {

                Token::SelfClosing(str) => {
                    element.tag = Self::parse_tag(&str);
                    element.children = None;
                    element.attr = Self::parse_attributes(&str);
                    element.content = None;
                    elements.push(element);
                    element = Element::default();
                }

                /* Token::StartTag(str) => {
                    tag = Self::parse_tag(&str);
                    element.tag = tag;
                    element.attr = Self::parse_attributes(&str);
                    element.children = Self::parse_elements(tokens, Some(tag));
                },
                Token::Content(str) => element.content = Some(str.to_string()),
                Token::EndTag(str) => {
                    elements.push(element);
                    element = Element::default();
                }, */



                Token::Doctype(str) | Token::XML(str) => {
                    element.content = None;
                    element.children = None;
                    element.attr = Self::parse_attributes(&str);
                    element.tag = Self::parse_tag(&str);
                    elements.push(element);
                    element = Element::default();
                },

                Token::PHP(str) => todo!(),

                _ => {},
            }

            *tokens = tokens[1..].to_vec();
        }

        if elements.len() > 0 {
            return Some(elements)
        }else{
            return None;
        }
    }
    
}
