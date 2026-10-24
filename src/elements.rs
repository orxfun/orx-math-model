use alloc::vec::Vec;

#[derive(Default, Debug)]
pub struct ElementsOfIdx {
    data: Vec<usize>,
}

impl ElementsOfIdx {
    pub fn fill_with(&mut self, values: impl IntoIterator<Item = usize>) {
        self.data.clear();
        self.data.extend(values);
    }

    pub fn retain(&mut self, filter: impl Fn(usize) -> bool) {
        self.data.retain(|x| filter(*x));
    }

    pub fn as_slice(&self) -> &[usize] {
        &self.data
    }
}
