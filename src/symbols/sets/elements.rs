use alloc::vec::Vec;

pub struct Elements(Vec<usize>);

impl Elements {
    pub fn fill(&mut self, len: Option<usize>, elements: impl Iterator<Item = usize>) {
        self.0.clear();

        if let Some(len) = len {
            self.0.reserve(len);
        }

        self.0.extend(elements);
    }
}
