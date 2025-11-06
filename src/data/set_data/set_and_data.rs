use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::data::SetGen;
use crate::Set;

pub trait SetAndData<'m, const N: usize> {
    fn set(&self) -> Set<'m, N>;

    fn set_gen(&self) -> &impl SetGen<N>;

    fn xyz(&self, set_depths: &SetDepths, index_values: &IndexValues) {
        // let values: Vec<_> = di
        //     .set_gen()
        //     .elements(di.set(), &set_depths, &index_values)
        //     .collect();
    }
}
