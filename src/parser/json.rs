use crate::utils::Shift;
pub struct Json {
    string: String,
    json_vec: Option<Vec<String>>

} 

impl Json {
    pub fn parse(string: String){
        let mut vectorized: Vec<String> = Self::vectorize_string(string);
    }

    fn vectorize_string(string: String) -> Vec<String> {
        let mut bytes: Vec<u8> = string.bytes().collect::<Vec<u8>>();
        let mut vectorized: Vec<String> = Vec::new();


        while bytes.len() > 0 {
            let mut byte = bytes[0];
            if byte == b'[' {
                println!("parse as array");
                let mut this_string = String::new();

                while bytes.len() > 0 && bytes[0] != b']' {
                    this_string.push(bytes[0] as char);
                    bytes.shift();
                }

                let vectorized = Self::vectorize_string(this_string);
                println!("nested arr: {:?}", vectorized);

            }else if byte == b'{' {
                println!("parse as object");
                let mut this_string = String::new();

                while bytes.len() > 0 && bytes[0] != b'}' {
                    this_string.push(bytes[0] as char);
                    println!("{}", bytes[0]);
                    bytes.shift();
                }

                let vectorized = Self::vectorize_string(this_string);
                println!("nested obj: {:?}", vectorized);


            }else{
                
                while bytes.len() > 0 {
                    byte == bytes[0];
                    let mut ident: String = String::new();
                    if byte == b':' || byte == b',' {
                        vectorized.push(ident);
                    }else {
                        ident.push(byte as char);
                    }

                    bytes.shift();
                }
            }

            bytes.shift();
        }

        todo!();
    }

    
}

impl<T> Shift<T> for Vec<T>{
    fn shift(&mut self) -> () where T: Clone {
        *self = self[1..].to_vec();
    }
}
