use alloc::vec::Vec;

const MAX_DEPTH: usize = 32;

pub struct Elements {
    elements: [Vec<usize>; MAX_DEPTH],
}
