use crate::data::set_data::indices::{IndexValues, IndexValuesIter, SetDepths};
use crate::symbols::sets::SetCore;
use crate::SetGen;
use alloc::boxed::Box;
use orx_self_or::SoR;

pub struct FunSet<'d, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> I,
{
    data: &'d Data,
    fun: F,
}

impl<'d, Data, I, T, F> FunSet<'d, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> I,
{
    pub fn d0(
        data: &'d Data,
        fun: impl Fn(&'d Data) -> I,
    ) -> FunSet<'d, Data, I, T, impl Fn(&'d Data, IndexValuesIter<'_>) -> I> {
        let fun = move |data: &'d Data, indices: IndexValuesIter<'_>| {
            debug_assert_eq!(indices.len(), 0);
            fun(data)
        };
        FunSet { data, fun }
    }

    pub fn d1(
        data: &'d Data,
        fun: impl Fn(&'d Data, usize) -> I,
    ) -> FunSet<'d, Data, I, T, impl Fn(&'d Data, IndexValuesIter<'_>) -> I> {
        let fun = move |data: &'d Data, mut indices: IndexValuesIter<'_>| {
            debug_assert_eq!(indices.len(), 1);
            let i1 = indices.next().unwrap();
            fun(data, i1)
        };
        FunSet { data, fun }
    }
}

// impl<'d, 'm, Data, I, T, F> SetGen<'m> for FunSet<'d, Data, I, T, F>
// where
//     I: IntoIterator<Item = T>,
//     T: SoR<usize>,
//     F: Fn(&'d Data, &[usize]) -> I,
// {
//     fn elements(
//         &self,
//         set: SetCore<'m>,
//         depths: &SetDepths<'m>,
//         index_values: &IndexValues,
//     ) -> Box<dyn Iterator<Item = usize> + '_> {
//         let dep_sets = set.depending_sets_core();
//         let depths = dep_sets.map(|s| depths.depth_of(s));
//         let indices = depths.map(|d| index_values[d]);
//         todo!()
//     }
// }
