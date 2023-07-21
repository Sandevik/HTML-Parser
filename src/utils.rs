pub trait Shift<T> {
    fn shift(&mut self) -> () where T: Clone;
}

pub fn clean_string(str: &str) -> String {
    str.chars().filter(|ch| *ch != '\"').collect::<String>()
}