use crate::symbols::sets::index_values::IndexValues;

pub trait SetGen {
    /// TODO: set_values as usize is a placeholder
    fn elements(&self, index_values: &mut IndexValues) -> Option<&[usize]>;
}

pub trait IntoSetGen {
    type SetGen: SetGen;

    fn into_set_gen(self) -> Self::SetGen;
}
