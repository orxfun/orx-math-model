use alloc::vec::Vec;

pub struct Elements(Vec<usize>);

impl Elements {
    fn prepare(&mut self, len: Option<usize>) {
        self.0.clear();

        if let Some(len) = len {
            self.0.reserve(len);
        }
    }

    pub fn fill_from_iter(&mut self, len: Option<usize>, elements: impl Iterator<Item = usize>) {
        self.prepare(len);
        self.0.extend(elements);
    }

    pub fn fill_from_slice(&mut self, slice: &[usize]) {
        self.prepare(Some(slice.len()));
        self.0.extend_from_slice(slice);
    }
}
