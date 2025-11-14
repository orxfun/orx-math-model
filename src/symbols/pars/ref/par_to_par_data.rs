use crate::data::{FunParData, Number};
use crate::symbols::pars::{Par, ParCore};

impl<'m> Par<'m, 0> {
    pub fn data<'d, Data, N, F>(
        self,
        data: &'d Data,
        fun: F,
    ) -> FunParData<'d, 'm, 0, Data, N, impl Fn(&'d Data, &[usize]) -> N>
    where
        N: Number,
        F: Fn(&'d Data) -> N,
    {
        let par = ParCore::from(self);

        let fun = move |data: &'d Data, indices: &[usize]| {
            debug_assert_eq!(indices.len(), 0);
            fun(data)
        };

        FunParData::new(par, data, fun)
    }
}

impl<'m> Par<'m, 1> {
    pub fn data<'d, Data, N, F>(
        self,
        data: &'d Data,
        fun: F,
    ) -> FunParData<'d, 'm, 1, Data, N, impl Fn(&'d Data, &[usize]) -> N>
    where
        N: Number,
        F: Fn(&'d Data, usize) -> N,
    {
        let par = ParCore::from(self);

        let fun = move |data: &'d Data, indices: &[usize]| {
            debug_assert_eq!(indices.len(), 1);
            let i0 = indices[0];
            fun(data, i0)
        };

        FunParData::new(par, data, fun)
    }
}

impl<'m> Par<'m, 2> {
    pub fn data<'d, Data, N, F>(
        self,
        data: &'d Data,
        fun: F,
    ) -> FunParData<'d, 'm, 2, Data, N, impl Fn(&'d Data, &[usize]) -> N>
    where
        N: Number,
        F: Fn(&'d Data, usize, usize) -> N,
    {
        let par = ParCore::from(self);

        let fun = move |data: &'d Data, indices: &[usize]| {
            debug_assert_eq!(indices.len(), 2);
            let i0 = indices[0];
            let i1 = indices[1];
            fun(data, i0, i1)
        };

        FunParData::new(par, data, fun)
    }
}

impl<'m> Par<'m, 3> {
    pub fn data<'d, Data, N, F>(
        self,
        data: &'d Data,
        fun: F,
    ) -> FunParData<'d, 'm, 3, Data, N, impl Fn(&'d Data, &[usize]) -> N>
    where
        N: Number,
        F: Fn(&'d Data, usize, usize, usize) -> N,
    {
        let par = ParCore::from(self);

        let fun = move |data: &'d Data, indices: &[usize]| {
            debug_assert_eq!(indices.len(), 3);
            let i0 = indices[0];
            let i1 = indices[1];
            let i2 = indices[2];
            fun(data, i0, i1, i2)
        };

        FunParData::new(par, data, fun)
    }
}

impl<'m> Par<'m, 4> {
    pub fn data<'d, Data, N, F>(
        self,
        data: &'d Data,
        fun: F,
    ) -> FunParData<'d, 'm, 4, Data, N, impl Fn(&'d Data, &[usize]) -> N>
    where
        N: Number,
        F: Fn(&'d Data, usize, usize, usize, usize) -> N,
    {
        let par = ParCore::from(self);

        let fun = move |data: &'d Data, indices: &[usize]| {
            debug_assert_eq!(indices.len(), 4);
            let i0 = indices[0];
            let i1 = indices[1];
            let i2 = indices[2];
            let i3 = indices[3];
            fun(data, i0, i1, i2, i3)
        };

        FunParData::new(par, data, fun)
    }
}
