use std::ops::Deref;

pub struct CurrentSetElements(Vec<usize>);

impl CurrentSetElements {
    pub(crate) fn at(&self, index: usize) -> usize {
        self.0[index]
    }
}

impl Deref for CurrentSetElements {
    type Target = [usize];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
