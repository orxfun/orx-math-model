use alloc::vec::Vec;

#[derive(Default)]
pub struct CurrentSetElements(Vec<usize>);

impl CurrentSetElements {
    pub fn new() -> Self {
        Default::default()
    }
}
