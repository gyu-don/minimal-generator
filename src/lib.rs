use std::{rc::Rc, cell::RefCell, fmt::Debug, future::Future, pin::Pin, task::{Context, Poll}};
mod waker;
mod iter;

#[derive(Debug)]
pub enum GeneratorState<T> {
    Yielded(T),
    Complete,
}

// yield_する側は、resumeする側に、Airlockを介してデータを受け渡す
type Airlock<T> = Rc<RefCell<Option<T>>>;
pub struct Co<T> (Airlock<T>);

impl<T> Co<T> {
    pub fn new(airlock: Airlock<T>) -> Self {
        Self(airlock)
    }

    pub fn yield_(&mut self, value: T) -> impl Future<Output=()> + '_ {
        self.0.replace(Some(value));
        Barrier(&self.0)
    }
}

pub struct Gen<T, F> {
    airlock: Airlock<T>,
    future: Pin<Box<F>>,
}

impl<T, F: Future> Gen<T, F> {
    pub fn new(producer: impl FnOnce(Co<T>) -> F) -> Self {
        let airlock = Airlock::default();
        let future = Box::pin(producer(Co::new(airlock.clone())));
        Self { airlock, future }
    }

    pub fn resume(&mut self) -> GeneratorState<T> {
        self.airlock.replace(None);
        advance(self.future.as_mut(), &self.airlock)
    }
}

// futureをpollすることでジェネレータの次の状態を得て、返す
fn advance<T, F: Future>(future: Pin<&mut F>, airlock: &Airlock<T>) -> GeneratorState<T> {
    // futureからpollをするのに、Contextが必要。Contextを作るのにWakerが必要。Wakerはほぼ何もしないWakerを用意する。
    let waker = waker::create();
    let mut cx = Context::from_waker(&waker);

    match future.poll(&mut cx) {
        Poll::Ready(_) => {
            airlock.replace(None);
            GeneratorState::Complete
        },
        Poll::Pending => {
            match airlock.replace(None) {
                Some(v) => GeneratorState::Yielded(v),
                None => unreachable!(),
            }
        },
    }
}

pub struct Barrier<'a, T>(&'a Airlock<T>);
impl<'a, T> Future for Barrier<'a, T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0.as_ref().borrow().is_some() {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}