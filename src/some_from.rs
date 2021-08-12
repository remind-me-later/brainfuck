pub trait SomeFrom<T>: Sized {
    fn some_from(item: T) -> Option<Self>;
}
