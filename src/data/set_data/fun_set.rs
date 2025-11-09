use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::symbols::sets::SetCore;
use crate::SetGen;
use alloc::boxed::Box;
use orx_self_or::SoR;

pub struct FunSet<'d, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, &[usize]) -> I,
{
    data: &'d Data,
    fun: F,
}

impl<'d, Data, I, T, F> FunSet<'d, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, &[usize]) -> I,
{
    pub fn d0(
        data: &'d Data,
        fun: impl Fn(&'d Data) -> I,
    ) -> FunSet<'d, Data, I, T, impl Fn(&'d Data, &[usize]) -> I> {
        let fun = move |data: &'d Data, _: &[usize]| fun(data);
        FunSet { data, fun }
    }

    pub fn d1(
        data: &'d Data,
        fun: impl Fn(&'d Data, usize) -> I,
    ) -> FunSet<'d, Data, I, T, impl Fn(&'d Data, &[usize]) -> I> {
        let fun = move |data: &'d Data, indices: &[usize]| fun(data, indices[0]);
        FunSet { data, fun }
    }
}

impl<'d, 'm, Data, I, T, F> SetGen<'m> for FunSet<'d, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, &[usize]) -> I,
{
    fn elements(
        &self,
        set: SetCore<'m>,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let dep_sets = set.depending_sets_core();
        let depths = dep_sets.map(|s| depths.depth_of(s));
        let indices = depths.map(|d| index_values[d]);
        todo!()
    }
}
