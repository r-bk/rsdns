pub trait Reader<T> {
    fn read(&mut self) -> crate::Result<T>;
}
