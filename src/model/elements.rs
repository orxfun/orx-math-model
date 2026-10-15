use alloc::vec::Vec;

pub struct Elements {
    data: Vec<usize>,
}

impl Elements {
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
