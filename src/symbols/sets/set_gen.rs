use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};

pub trait SetGen {
    fn set_elements<'m>(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        current_indices: &IndexValues,
        elements: &'m mut Elements<'m>,
    );
}

pub trait IntoSetGen {
    type SetGen: SetGen;

    fn into_set_gen(self) -> Self::SetGen;
}
