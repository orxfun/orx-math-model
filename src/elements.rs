use alloc::vec::Vec;

#[derive(Default, Debug)]
pub struct Elements {
    data: Vec<usize>,
}

impl Elements {
    pub fn fill_with(&mut self, values: impl IntoIterator<Item = usize>) {
        self.data.clear();
        self.data.extend(values);
    }

    pub fn as_slice(&self) -> &[usize] {
        &self.data
    }
}
