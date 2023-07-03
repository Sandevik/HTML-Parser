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
            ch: str.bytes().collect::<Vec<u8>>()[0] as char
        }
    }
    
    pub fn eat(&mut self) -> char {
        if self.pos < self.size {
            self.pos += 1;
            self.ch = self.buf[self.pos] as char;
        }else{
            self.pos = self.size;
        }
        return self.ch
    }
}

pub struct Tokenizer {}
impl Tokenizer {
    pub fn tokenize(consumer: Consumer) {

    }
}