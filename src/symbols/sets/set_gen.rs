pub enum Elements<'a> {
    Owned(&'a [usize]),
    Storage,
}

pub trait SetGen {
    /// TODO: set_values as usize is a placeholder
    /// Takes storage, returns (elements, storage) tuple.
    fn elements<'a>(&'a self, set_values: usize, storage: &'a mut [usize]) -> Elements<'a>;
}
