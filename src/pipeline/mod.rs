pub mod adaptor;

pub struct Data<T> {
    pub data: T,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Data<T> {
        Data { data }
    }

    pub fn adapt<P, O>(self, process: P) -> Data<O>
    where
        P: Fn(T) -> O,
    {
        Data::new(process(self.data))
    }
}

pub trait Process<I, O> {
    fn process(&self, input: I) -> O;
}
