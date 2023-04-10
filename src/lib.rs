use std::{rc::Rc, cell::Cell, future::Future};

pub struct Co<T> (Rc<Cell<Option<T>>>);

impl<T> Co<T> {
    pub fn yield_(&mut self, value: T) -> impl Future<Output=()> + '_ {
        DummyFuture()
    }
}

pub struct DummyFuture();
impl Future for DummyFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        unimplemented!();
    }
}