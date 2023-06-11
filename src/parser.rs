#![allow(dead_code, unused_variables, unused_mut)]

#[derive(Debug, PartialEq)]
pub enum ElementType {
    DocType,
    Html,
    HeadOpen,
    HeadClose,
    Meta,
    BodyOpen,
    BodyClose,
    Link,
    Script,
    Style,
    Title,

    Div,
    Span,

    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    A,

    Main,
    Section,
    Article,
    Image,
    Header,
    Nav,
    Footer,
    Aside,
    Iframe,

    Button,
    Input,
    TextArea,
    Form,
    Option,
    Select,

    NoScript,

    Table,
    TBody,
    Td,
    Th,
    Tr,

    Ul,
    Ol,
    Li,

    Illegal, // For those that are not yet implemented
}

#[derive(Debug)]
pub struct HTMLElementTree<'a> {
    root: HTMLElement<'a>,
    children: Option<Vec<HTMLElement<'a>>>,
}

#[derive(Debug)]
pub struct HTMLElement<'a> {
    inner_text: Option<&'a str>,
    classes: Option<Vec<&'a str>>,
    r#type: Option<ElementType>,
}

#[derive(Debug)]
pub struct HTMLParser<'a> {
    content: Option<String>,
    tree: Option<HTMLElementTree<'a>>,
}

impl<'a> HTMLParser<'a> {
    pub fn new() -> HTMLParser<'a> {
        HTMLParser {
            content: None,
            tree: None,
        }
    }

    pub fn read(&mut self, content: String) {
        self.content = Some(content);
        
    }



    pub fn parse(str: &'a str) {        
        println!("Initial string: {}", &str);
        let mut string = String::from(str.clone());
        let mut bytes: Vec<u8> = string.bytes().collect::<Vec<u8>>();
        let mut tree: Vec<String> = Vec::new();
        
        while bytes.len() > 0 {
            let (content, new_bytes) = Self::vectorize_content(bytes); 
            bytes = new_bytes;
            if content != "" && content != " "  {
                tree.push(content);
            } 
        
            let (tag, new_bytes) = Self::vectorize_tag(bytes);
            bytes = new_bytes;
            tree.push(tag);
        
        }

        
        println!("{:#?}", tree);
    }

    fn vectorize_content(mut bytes: Vec<u8>) -> (String, Vec<u8>) {
        let mut content: Vec<u8> = Vec::new(); 
        let mut idx = 0;
        while idx < bytes.len() {
            let current = bytes[idx];
            if current == b'<'{
                break;
            }

            if current != b'>' {
                content.push(current);
            }
            
            idx += 1;
        }
        let string = String::from_utf8(content).unwrap();
        return (string, bytes[idx..].to_vec());
    }

    fn vectorize_tag(bytes: Vec<u8>) -> (String, Vec<u8>) {
        let mut tag: String = String::from("");
        let mut idx = 0;
        
        while idx < bytes.len() && bytes[0] == b'<' {
            let current = bytes[idx];
            if current == b'>' {
                break;
            }
            idx += 1;
        }

        if idx != 0 {
            tag = String::from_utf8(bytes[0..idx + 1].to_vec()).unwrap_or("".to_owned());
        }
        
        return (tag, bytes[idx..].to_vec());
    }




}
