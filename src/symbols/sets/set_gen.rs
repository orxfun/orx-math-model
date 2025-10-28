use alloc::vec::Vec;

pub trait SetGen {
    /// TODO: set_values as usize is a placeholder
    /// Takes storage, returns (elements, storage) tuple.
    fn elements(&self, set_values: usize, storage: Vec<usize>) -> (Vec<usize>, Vec<usize>);
}
