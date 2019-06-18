pub trait Process<I, O> {
    fn adaptor(&self, I) -> O;
}

pub struct Data<T> {
    data: T,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Data<T> {unimplemented!();}

    pub fn adapt<P, O>(self, process: P) -> Data<O>
        where P: Fn(T) -> O {
        unimplemented!();
    }
}