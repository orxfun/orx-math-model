use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};

pub trait SetGen<'m> {
    fn set_elements(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        index_values: &IndexValues,
        elements: &'m mut Elements<'m>,
    );
}
