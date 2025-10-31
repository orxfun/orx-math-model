use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};

pub trait SetGen {
    fn set_elements<'m>(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        index_values: &IndexValues,
        elements: &'m mut Elements<'m>,
    );
}
