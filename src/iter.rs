use crate::{Gen, GeneratorState};
use std::future::Future;

impl<T, F: Future<Output=()>> IntoIterator for Gen<T, F> {
    type Item = T;
    type IntoIter = IntoIter<T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T, F> (Gen<T, F>);

impl<T, F: Future<Output=()>> Iterator for IntoIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete => None,
        }
    }
}