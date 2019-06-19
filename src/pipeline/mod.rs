pub mod adaptor;

pub struct Data<T> {
    pub data: T,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Data<T> {
        Data { data }
    }

    pub fn adapt<P, O>(self, processor: P) -> Data<O>
    where
        P: Processor<T, O>,
    {
        Data::new(processor.process(self.data))
    }
}

pub trait Processor<I, O> {
    fn process(&self, input: I) -> O;
}

pub struct Process<I, O, P: Fn(I)->O>
{
    process_impl: P,
}

impl<P> Process<P>
where
    P: Fn(I) -> O,
{
    pub fn new(process: P) -> Process<P> {
        Process {
            process_impl: process,
        }
    }
}

impl<P, I, O> Processor<I, O> for Process<P>
where
    P: Fn(I) -> O,
{
    fn process(&self, input: I) -> O {
        self.process_impl(input)
    }
}
