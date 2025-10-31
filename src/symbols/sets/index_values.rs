use alloc::vec;
use alloc::vec::Vec;

pub struct IndexValues {
    values: Vec<usize>,
    max_depth: usize,
    current_depth: usize,
    elements: Vec<Vec<usize>>,
}

impl IndexValues {
    pub fn new(max_depth: usize) -> Self {
        Self {
            values: vec![usize::MAX; max_depth],
            max_depth,
            current_depth: 0,
            elements: vec![vec![]; max_depth],
        }
    }

    pub fn current_depth(&self) -> usize {
        self.current_depth
    }

    // set elements

    fn prepare(&mut self, depth: usize, len: Option<usize>) {
        while self.elements.len() <= depth {
            self.elements.push(Vec::new());
        }

        let vec = &mut self.elements[depth];
        vec.clear();

        if let Some(len) = len {
            vec.reserve(len);
        }
    }

    pub fn fill_elements_from_iter(
        &mut self,
        depth: usize,
        len: Option<usize>,
        elements: impl Iterator<Item = usize>,
    ) {
        self.prepare(depth, len);
        self.elements[depth].extend(elements);
    }
}
