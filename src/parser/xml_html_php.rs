#![allow(dead_code, unused_variables, unused_assignments, unused_mut)]

use std::rc::Rc;

#[derive(Debug)]
pub struct Consumer {
    buf: Rc<[u8]>,
    pos: usize,
    size: usize,
    ch: char
}
impl Consumer {
    pub fn from(str: &str) -> Consumer {
        Consumer {
            buf: str.bytes().collect::<Rc<[u8]>>(),
            pos: 0,
            size: str.bytes().len(),
            ch: str.chars().nth(0).unwrap()
        }
    }

    pub fn eat(&mut self) -> char {
        if self.pos + 1 < self.size {
            self.pos += 1;
            self.ch = self.buf[self.pos] as char;
        }else{
            self.pos = self.size;
        }
        return self.ch
    }
    pub fn peek(&self) -> char {
        if self.pos + 1 < self.size {
            return self.buf[self.pos + 1] as char;
        }else {
            return 0 as char;
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Doctype(String),
    XML(String),
    PHP(String),
    StartTag(String),
    EndTag(String),
    SelfClosing(String),
    Comment(String),
    String(String),
    EOF
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
    PHP
}

pub struct Tokenizer {}
impl Tokenizer {
    pub fn tokenize(mut consumer: Consumer) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut token_type: TokenType = TokenType::Unknown;
        let mut ident: String = String::new();

        while consumer.pos < consumer.size {
            match consumer.ch {
                '<' => {
                    if ident.len() > 0 {
                        tokens.push(Token::String(ident));
                        ident = String::new();
                    }
                    if consumer.peek() == '/' {
                        //closing tag
                        token_type = TokenType::EndTag;
                        ident.push(consumer.ch);
                    }else if consumer.peek() == '!' {
                        if consumer.buf[consumer.pos + 2] == b'D' {
                            //Doctype
                            token_type = TokenType::Doctype;
                        }else{
                            // comment
                            token_type = TokenType::Comment;
                        }
                        ident.push(consumer.ch);
                    } else if consumer.peek() == '?' {
                        //xml declatation
                        /* TODO: Add functionality to parse PHP */
                        if consumer.buf[consumer.pos + 2] != b'p' && consumer.buf[consumer.pos + 2] != b'=' {
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
                    }else{
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
                        TokenType::Unknown => Token::String(copy_ident),
                        TokenType::PHP => Token::PHP(copy_ident),
                    };
                    tokens.push(token);
                    ident = String::new();
                }

                
                _ => ident.push(consumer.ch)
            }

            consumer.eat();
        
        }

        return tokens;

    }
}