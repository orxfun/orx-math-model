use alloc::vec::Vec;

#[derive(Default, Debug, PartialEq)]
pub struct ElementContainer<T> {
    data: Vec<T>,
}

impl<T> ElementContainer<T> {
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
}
