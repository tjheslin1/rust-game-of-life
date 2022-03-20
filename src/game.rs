pub trait Game<T> {
    fn next(&self) -> T;
}
