#![allow(dead_code, unused_variables, unused_assignments, unused_mut)]

use std::{collections::HashMap, rc::Rc};

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

#[derive(Debug, PartialEq)]
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
}

pub type Attributes = Vec<HashMap<String, String>>;

#[derive(Debug, PartialEq)]
pub struct Element {
    tag: Tag,
    content: Option<String>,
    attr: Option<Attributes>,
}
impl Element {
    pub fn default() -> Element {
        Element {
            tag: Tag::Unknown,
            content: None,
            attr: None,
        }
    }

    pub fn new(tag: Tag, content: Option<String>, attr: Option<Attributes>) -> Element {
        Element {
            tag: tag,
            content: content,
            attr: attr,
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

    pub fn parse_elements(consumer: Consumer) -> Vec<Element> {
        let mut elements = Vec::<Element>::new();
        let mut tokens = Self::tokenize(consumer);
        let mut element: Element = Element::default();

        while tokens.len() > 0 {
            match &tokens[0] {
                Token::Doctype(str) => element = Element {tag: Tag::Doctype, content: None, attr: Self::parse_attributes(str)},
                Token::Content(str) => element = Element {tag: element.tag, content: Some(str.to_string()), attr: element.attr},
                Token::StartTag(str) => element = Element {tag: Self::parse_tag(str), content: None, attr: Self::parse_attributes(str)},
                Token::EndTag(str) => {
                    elements.push(element);
                    element = Element::default();
                }
                Token::SelfClosing(str) => {
                    if element != Element::default() {
                        elements.push(element);
                        element = Element::default();
                    }
                    elements.push(Element::new(Self::parse_tag(str), None, Self::parse_attributes(str)));
                }
                Token::PHP(str) => elements.push(Self::parse_php_tag(str)),
                Token::XML(str) => elements.push(Element::new(Tag::XML, None, Self::parse_attributes(str))),

                Token::Comment(str) => {}
                Token::EOF => {}
            }




            tokens = tokens[1..].to_vec();
        }
        return elements;
    }


    fn parse_attributes(str: &str) -> Option<Attributes> {

        return None;
    }

    fn parse_tag(str: &str) -> Tag {
        return Tag::Unknown;
    }

    fn parse_php_tag(str: &str) -> Element {

        return Element::default();
    }
    
}
