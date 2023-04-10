use std::{rc::Rc, cell::Cell, future::Future, pin::Pin};

type Airlock<T> = Rc<Cell<Option<T>>>;
pub struct Co<T> (Airlock<T>);

impl<T> Co<T> {
    pub fn new(airlock: Airlock<T>) -> Self {
        Self(airlock)
    }

    pub fn yield_(&mut self, value: T) -> impl Future<Output=()> + '_ {
        DummyFuture()
    }
}

pub struct Gen<T, F: Future> {
    airlock: Airlock<T>,
    future: Pin<Box<F>>,
}

impl<T, F: Future> Gen<T, F> {
    pub fn new(producer: impl FnOnce(Co<T>) -> F) -> Self {
        let airlock = Airlock::default();
        let future = Box::pin(producer(Co::new(airlock.clone())));
        Self { airlock, future }
    }
}

pub struct DummyFuture();
impl Future for DummyFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        unimplemented!();
    }
}
