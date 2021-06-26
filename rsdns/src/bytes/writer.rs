pub trait Writer<T> {
    fn write(&mut self, o: &T) -> crate::ProtocolResult<usize>;
}
