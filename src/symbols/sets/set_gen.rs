use crate::symbols::sets::elements::Elements;

pub trait SetGen {
    /// TODO: set_values as usize is a placeholder
    /// Fills storage with elements represented as usize.
    fn elements(&self, set_values: usize, storage: &mut Elements);
}
