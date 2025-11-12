use crate::data::set_data::indices::{IndexValues, IndexValuesIter, SetDepths};
use crate::symbols::sets::SetCore;
use crate::SetAndData;
use alloc::boxed::Box;
use orx_self_or::SoR;

#[derive(Debug)]
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

impl<'d, 'm, Data, I, T, F> SetAndData<'m> for FunSetAndData<'d, 'm, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> I,
{
    fn set(&self) -> SetCore<'m> {
        self.set
    }

    fn elements(
        &'m self,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let model = self.set.symbol().model;
        let depending_indices = self.set.sym_data().depends_on_indices();
        let indices = IndexValuesIter::new(model, depending_indices, depths, index_values);
        let elements = (self.fun)(self.data, indices);
        let elements = elements.into_iter().map(|x| *x.get_ref());
        Box::new(elements)
    }
}
