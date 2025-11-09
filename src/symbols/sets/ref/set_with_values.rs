use crate::data::{FunSetAndData, FunSetAndDataD0, FunSetAndDataD1, IndexValuesIter};
use crate::symbols::sets::SetCore;
use crate::Set;
use orx_self_or::SoR;

impl<'m> Set<'m, 0> {
    pub fn data<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: fn(&'d Data) -> I,
    ) -> FunSetAndDataD0<'m, 'd, Data, I, T>
    where
        I: IntoIterator<Item = T>,
        T: SoR<usize>,
    {
        FunSetAndDataD0::new(self, data, fun)
    }

    pub fn data_new<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: impl Fn(&'d Data) -> I,
    ) -> FunSetAndData<'d, 'm, Data, I, T, impl Fn(&'d Data, IndexValuesIter<'_>) -> I>
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
}

impl<'m> Set<'m, 1> {
    pub fn data<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: fn(&'d Data, usize) -> I,
    ) -> FunSetAndDataD1<'m, 'd, Data, I, T>
    where
        I: IntoIterator<Item = T>,
        T: SoR<usize>,
    {
        FunSetAndDataD1::new(self, data, fun)
    }

    pub fn data_new<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: impl Fn(&'d Data, usize) -> I,
    ) -> FunSetAndData<'d, 'm, Data, I, T, impl Fn(&'d Data, IndexValuesIter<'_>) -> I>
    where
        I: IntoIterator<Item = T>,
        T: SoR<usize>,
    {
        let set = SetCore::from(self);
        debug_assert_eq!(set.dim(), 1);

        let fun = move |data: &'d Data, mut indices: IndexValuesIter<'_>| {
            debug_assert_eq!(indices.len(), 1);
            let i1 = indices.next().unwrap();
            fun(data, i1)
        };

        FunSetAndData::new(set, data, fun)
    }
}
