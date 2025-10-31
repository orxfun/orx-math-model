use crate::symbols::sets::indices::Depth;
use alloc::vec::Vec;

const MAX_DEPTH: usize = 32;

pub struct Elements<'a> {
    independent_elements: [&'a [usize]; MAX_DEPTH],
    stored_elements: [Vec<usize>; MAX_DEPTH],
}

impl<'a> Elements<'a> {
    pub fn parent_elements<'b>(&self, current_depth: Depth, parent_depth: Depth) -> &'b [usize] {
        debug_assert!(current_depth > parent_depth);
        let parent_elements = match self.independent_elements[parent_depth.depth()].len() {
            0 => &self.stored_elements[parent_depth.depth()],
            _ => self.independent_elements[parent_depth.depth()],
        };
        let (ptr, len) = (parent_elements.as_ptr(), parent_elements.len());
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }

    pub fn set_independent_elements(&mut self, depth: Depth, elements: &'a [usize]) {
        self.independent_elements[depth.depth()] = elements;
    }

    pub fn set_stored_elements(&mut self, depth: Depth, elements: impl IntoIterator<Item = usize>) {
        let vec = &mut self.stored_elements[depth.depth()];
        vec.clear();
        vec.extend(elements);
    }
}
