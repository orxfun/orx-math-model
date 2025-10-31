use crate::symbols::sets::{
    indices::{Depth, Elements, IndexValues},
    set_gen::SetGen,
};

pub struct Subset<S, F>
where
    S: SetGen,
    F: Fn(usize) -> bool,
{
    set: S,
    filter: F,
}

impl<S, F> SetGen for Subset<S, F>
where
    S: SetGen,
    F: Fn(usize) -> bool,
{
    fn set_elements(&self, depth: Depth, current_indices: &IndexValues, elements: &mut Elements) {}
}
