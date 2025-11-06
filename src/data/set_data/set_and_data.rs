use crate::{data::set_data::set_gen::SetGen, Set};

pub trait SetAndData<'m, const N: usize> {
    fn set(&self) -> Set<'m>;

    fn data(&self) -> &impl SetGen<N>;
}
