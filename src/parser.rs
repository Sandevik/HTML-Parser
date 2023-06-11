#![allow(dead_code, unused_variables, unused_mut)]

#[derive(Debug, PartialEq, Clone)]
pub enum ElementType {
    DocType,
    Html,
    Head,

    Meta,
    Body,
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
    Br,

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

#[derive(Debug, Clone)]
pub struct HTMLElement<'a> {
    inner_text: Option<&'a str>,
    children: Option<Vec<HTMLElement<'a>>>,
    r#type: Option<ElementType>,
}

impl<'a> HTMLElement<'a> {
    pub fn new(r#type: ElementType) -> HTMLElement<'a> {
        HTMLElement {
            inner_text: None,
            children: None,
            r#type: Some(r#type),
        }
    }
    pub fn default() -> HTMLElement<'a> {
        HTMLElement {
            inner_text: None,
            children: None,
            r#type: None,
        }
    }
}

#[derive(Debug)]
pub struct HTMLParser<'a> {
    content: Option<String>,
    vectorized: Option<Vec<String>>,
    tree: Option<HTMLElementTree<'a>>,
}

impl<'a> HTMLParser<'a> {
    pub fn new() -> HTMLParser<'a> {
        HTMLParser {
            content: None,
            vectorized: None,
            tree: None,
        }
    }

    pub fn read(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn parse(&mut self, str: &'a str) {
        Self::vectorize(self, str);
        Self::tokenize(self);

        /*
            Recursive
            _________


            kolla om tagg - regex

            kolla om tag inte är selfclosing - Regex
            Om selfclosing, lägg till som child
            kolla om nästa är content eller öppningstagg - Regex
            om öppningstagg, recursive()
            annars kolla om nästa är content eller stängningstagg - Regex
            om stängningstagg, skapa HTMLElement, och lägg i

        */
    }

    fn tokenize(&mut self) {
        //println!("{:#?}", self.vectorized);
        match &self.vectorized {
            Some(vector) => {
                use regex::Regex;
                let open_close = Regex::new("<\\/?[a-zA-Z0-9 =\"-]+>").expect("Regex is not valid");
                let self_closing: Regex = Regex::new("<(!DOCTYPE|img|base|br|embed|col|meta|param|input|hr|source|link|wbr|track|area)\\[A-Za-z0-9=\" -\\]+\\/?>").expect("Self-closing Regex not valid");

                let mut elements: Vec<HTMLElement> = Vec::new();

                let mut last_element: HTMLElement = HTMLElement::default();
                for (_, tag) in vector.iter().enumerate() {
                    if tag != "\r\n" && tag != " " && tag != "\t" && tag != "\n" && tag != "\r" {
                        // check selfclosing first
                        if self_closing.is_match(tag) {
                            if last_element.r#type.is_some() {
                                elements.push(last_element.clone());
                            }
                            last_element = Self::match_self_closing(tag)
                        } else if open_close.is_match(tag) {
                            elements.push(last_element.clone());
                            last_element = Self::match_open_close(tag);
                        } else {
                            //text is content
                            last_element.inner_text = Some(tag);
                        }
                    }
                }


                

                println!("162: {:#?}", elements);
            }
            None => todo!(),
        }
    }

    fn match_open_close(tag: &String) -> HTMLElement<'a> {
        let cleaned_tag: &Vec<&str> = &tag[1..].split(" ").collect::<Vec<&str>>();
        let element = match cleaned_tag[0] {
            "html" => HTMLElement::new(ElementType::Html),
            "head" => HTMLElement::new(ElementType::Head),
            "body" => HTMLElement::new(ElementType::Body),
            "script" => HTMLElement::new(ElementType::Script),
            "title" => HTMLElement::new(ElementType::Title),
            "div" => HTMLElement::new(ElementType::Div),
            "span" => HTMLElement::new(ElementType::Span),
            "h1" => HTMLElement::new(ElementType::H1),
            "h2" => HTMLElement::new(ElementType::H2),
            "h3" => HTMLElement::new(ElementType::H3),
            "h4" => HTMLElement::new(ElementType::H4),
            "h5" => HTMLElement::new(ElementType::H5),
            "h6" => HTMLElement::new(ElementType::H6),
            "a" => HTMLElement::new(ElementType::A),
            "p" => HTMLElement::new(ElementType::P),
            "main" => HTMLElement::new(ElementType::Main),
            "section" => HTMLElement::new(ElementType::Section),
            "article" => HTMLElement::new(ElementType::Article),
            "header" => HTMLElement::new(ElementType::Header),
            "nav" => HTMLElement::new(ElementType::Nav),
            "footer" => HTMLElement::new(ElementType::Footer),
            "aside" => HTMLElement::new(ElementType::Aside),
            "iframe" => HTMLElement::new(ElementType::Iframe),
            "button" => HTMLElement::new(ElementType::Button),
            "textarea" => HTMLElement::new(ElementType::TextArea),
            "form" => HTMLElement::new(ElementType::Form),
            "option" => HTMLElement::new(ElementType::Option),
            "select" => HTMLElement::new(ElementType::Select),
            "noscript" => HTMLElement::new(ElementType::NoScript),
            "table" => HTMLElement::new(ElementType::Table),
            "tbody" => HTMLElement::new(ElementType::TBody),
            "td" => HTMLElement::new(ElementType::Td),
            "tr" => HTMLElement::new(ElementType::Tr),
            "th" => HTMLElement::new(ElementType::Th),
            "ul" => HTMLElement::new(ElementType::Ul),
            "ol" => HTMLElement::new(ElementType::Ol),
            "li" => HTMLElement::new(ElementType::Li),

            _ => HTMLElement::new(ElementType::Illegal),
        };
        return element;
    }

    fn match_self_closing(tag: &String) -> HTMLElement<'a> {
        let cleaned_tag: &Vec<&str> = &tag[1..].split(" ").collect::<Vec<&str>>();
        let element = match cleaned_tag[0] {
            "!DOCTYPE" => HTMLElement::new(ElementType::DocType),
            "img" => HTMLElement::new(ElementType::Image),
            "br" => HTMLElement::new(ElementType::Br),
            "input" => HTMLElement::new(ElementType::Input),
            "meta" => HTMLElement::new(ElementType::Meta),

            //Fix more
            _ => HTMLElement::new(ElementType::Illegal),
        };
        return element;
    }

    fn vectorize(&mut self, str: &'a str) -> () {
        let mut string = String::from(str.clone());
        let mut bytes: Vec<u8> = string.bytes().collect::<Vec<u8>>();
        let mut tree: Vec<String> = Vec::new();

        while bytes.len() > 0 {
            let (content, new_bytes) = Self::vectorize_content(bytes);
            bytes = new_bytes;
            if content != "        " && content != "    " && content != ""{
                tree.push(content.trim().to_string());
            }

            let (tag, new_bytes) = Self::vectorize_tag(bytes);
            bytes = new_bytes;
            tree.push(tag);
        }

        self.vectorized = Some(tree);
    }

    fn vectorize_content(mut bytes: Vec<u8>) -> (String, Vec<u8>) {
        let mut content: Vec<u8> = Vec::new();
        let mut idx = 0;
        while idx < bytes.len() {
            let current = bytes[idx];
            if current == b'<' {
                break;
            }
            if current != b'>' && current != b'\n' &&  current != b'\r' && current != b'\t' {
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
