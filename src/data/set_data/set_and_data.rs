use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::data::SetGen;
use crate::Set;
use alloc::boxed::Box;

pub trait SetAndData<'m, const N: usize> {
    fn set(&self) -> Set<'m, N>;

    fn set_gen(&self) -> &impl SetGen<N>;

    fn elements(
        &'m self,
        set_depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        self.set_gen()
            .elements(self.set(), &set_depths, &index_values)
    }
}
