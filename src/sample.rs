use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sample<T>(Vec<T>);

impl<T> AsRef<[T]> for Sample<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> IntoIterator for Sample<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(Sample<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        (self.0).0.pop()
    }
}

impl<T> From<Vec<T>> for Sample<T> {
    fn from(vec: Vec<T>) -> Self {
        Sample(vec)
    }
}
