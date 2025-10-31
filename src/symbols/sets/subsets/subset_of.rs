use crate::symbols::sets::{index_values::IndexValues, set_gen::SetGen};

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
    fn elements(&self, index_values: &mut IndexValues) -> Option<&[usize]> {
        let set = self.set.elements(index_values);

        None
    }
}
