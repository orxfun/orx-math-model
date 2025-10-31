use crate::symbols::sets::elements::Elements;

pub trait SetGen {
    /// TODO: set_values as usize is a placeholder
    fn elements(&self, set_values: usize, storage: &mut Elements) -> Option<&[usize]>;
}

pub trait IntoSetGen {
    type SetGen: SetGen;

    fn into_set_gen(self) -> Self::SetGen;
}
