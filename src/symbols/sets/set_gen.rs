use crate::symbols::sets::indices::{Depth, Elements, IndexValues};

pub trait SetGen {
    fn set_elements<'a>(
        &'a self,
        depth: Depth,
        current_indices: &IndexValues,
        elements: &'a mut Elements<'a>,
    );
}

pub trait IntoSetGen {
    type SetGen: SetGen;

    fn into_set_gen(self) -> Self::SetGen;
}
