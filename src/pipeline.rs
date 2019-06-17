pub trait Process<I, O> {
    fn apply(&self, I) -> O;
}

pub struct Adaptor<T> {
    data: T,
}

impl<T> Adaptor<T> {
    pub fn new(data: T) -> Adaptor<T> {
        Adaptor {
            data: data
        }
    }

    pub fn adapt<P, O>(self, process: P) -> Adaptor<O>
        where P: Process<T, O> {
        Adaptor::new(process.apply(self.data))
    }
}