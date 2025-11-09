use crate::data::set_data::indices::{IndexValues, IndexValuesIter, SetDepths};
use crate::data::set_data::set_gen::SetGenNew;
use crate::symbols::sets::SetCore;
use alloc::boxed::Box;
use orx_self_or::SoR;

pub struct FunSetAndData<'d, 'm, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> I,
{
    set: SetCore<'m>,
    data: &'d Data,
    fun: F,
}

impl<'d, 'm, Data, I, T, F> FunSetAndData<'d, 'm, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> I,
{
    pub(crate) fn new(set: SetCore<'m>, data: &'d Data, fun: F) -> Self {
        Self { set, data, fun }
    }
}

impl<'d, 'm, Data, I, T, F> SetGenNew<'m> for FunSetAndData<'d, 'm, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> I,
{
    fn elements(
        &self,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let indices = IndexValuesIter::new(self.set, depths, index_values);
        let elements = (self.fun)(self.data, indices);
        let elements = elements.into_iter().map(|x| *x.get_ref());
        Box::new(elements)
    }
}
