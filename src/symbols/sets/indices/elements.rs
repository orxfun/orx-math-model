use alloc::vec::Vec;

use crate::symbols::sets::indices::Depth;

const MAX_DEPTH: usize = 32;

pub struct Elements<'a> {
    independent_elements: [&'a [usize]; MAX_DEPTH],
    stored_elements: [Vec<usize>; MAX_DEPTH],
}

impl<'a> Elements<'a> {
    pub fn set_independent_elements(&mut self, depth: Depth, elements: &'a [usize]) {
        self.independent_elements[depth.depth()] = elements;
    }
}
