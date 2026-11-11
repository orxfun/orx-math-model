use alloc::vec::Vec;
use core::{iter::Copied, slice::Iter};

#[derive(Default, Debug, PartialEq)]
pub struct Elements<T> {
    data: Vec<T>,
}

impl<T> From<Vec<T>> for Elements<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> Elements<T> {
    pub fn fill(&mut self, values: impl IntoIterator<Item = T>) {
        self.data.clear();
        self.data.extend(values);
    }

    pub fn retain(&mut self, filter: impl Fn(&T) -> bool) {
        self.data.retain(filter);
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn iter(&self) -> Copied<Iter<'_, T>>
    where
        T: Copy,
    {
        self.data.iter().copied()
    }
}
