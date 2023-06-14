pub trait Shift<T> {
    fn shift(&mut self) -> () where T: Clone;
}