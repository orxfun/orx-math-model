use crate::data::{EmptySet, FunSetAndData, IndexValuesIter};
use crate::symbols::sets::SetCore;
use crate::Set;
use orx_self_or::SoR;

impl<'m> Set<'m, 0> {
    pub fn data<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: impl Fn(&'d Data) -> I,
    ) -> FunSetAndData<'d, 'm, 0, Data, I, T, impl Fn(&'d Data, IndexValuesIter<'_>) -> I>
    where
        I: IntoIterator<Item = T>,
        T: SoR<usize>,
    {
        let set = SetCore::from(self);
        debug_assert_eq!(set.dim(), 0);

        let fun = move |data: &'d Data, indices: IndexValuesIter<'_>| {
            debug_assert_eq!(indices.len(), 0);
            fun(data)
        };

        FunSetAndData::new(set, data, fun)
    }

    pub fn data_empty(self) -> EmptySet<'m, 0> {
        EmptySet::new(self)
    }
}

impl<'m> Set<'m, 1> {
    pub fn data<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: impl Fn(&'d Data, usize) -> I,
    ) -> FunSetAndData<'d, 'm, 1, Data, I, T, impl Fn(&'d Data, IndexValuesIter<'_>) -> I>
    where
        I: IntoIterator<Item = T>,
        T: SoR<usize>,
    {
        let set = SetCore::from(self);
        debug_assert_eq!(set.dim(), 1);

        let fun = move |data: &'d Data, mut indices: IndexValuesIter<'_>| {
            debug_assert_eq!(indices.len(), 1);
            #[allow(clippy::missing_panics_doc, clippy::unwrap_used)]
            let i0 = indices.next().unwrap();
            fun(data, i0)
        };

        FunSetAndData::new(set, data, fun)
    }

    pub fn data_empty(self) -> EmptySet<'m, 1> {
        EmptySet::new(self)
    }
}
