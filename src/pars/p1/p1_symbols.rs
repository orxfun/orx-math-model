use crate::pars::p1::DP1;
use crate::sets::IndexCatInt;
use crate::val_idx_pairs::ValIdxPairD1;
use crate::{Ptr, Res, Set, SetCatInt, Symbol, Val, ValInt};
use alloc::boxed::Box;
use core::{marker::PhantomData, ops::Deref};

#[derive(derive_more::From, Clone, Copy)]
pub struct ParD1<'m, 'd, V: Val, I0: Set>(pub(crate) Symbol<'m, 'd, DP1<'d>>, PhantomData<(V, I0)>);

impl<'m, 'd, V: Val, I0: Set> From<Symbol<'m, 'd, DP1<'d>>> for ParD1<'m, 'd, V, I0> {
    fn from(value: Symbol<'m, 'd, DP1<'d>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<'m, 'd, V: Val, I0: Set> Deref for ParD1<'m, 'd, V, I0> {
    type Target = Symbol<'m, 'd, DP1<'d>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'m, 'd, V: Val, I0: Set> ParD1<'m, 'd, V, I0> {
    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Raw + 'd) -> Res
    where
        (V, I0): ValIdxPairD1<V = V, I0 = I0>,
    {
        let fun: Box<dyn Fn(I0::IdxRaw<'d>) -> V::Raw + 'd> = Box::new(fun);
        let data = unsafe { self.el.get() };
        data.bind::<V, I0>(fun)
    }

    // pub fn at(self, idx: impl Into<I0::IdxAny<'m, 'd>>) -> V::Sym<'m, 'd> {
    //     let idx = idx.into();
    //     todo!()
    // }
}

type V = ValInt;
type I0 = SetCatInt;
impl<'m, 'd> ParD1<'m, 'd, V, I0> {
    pub fn at(self, idx: impl Into<<I0 as Set>::IdxAny<'m, 'd>>) -> <V as Val>::Sym<'m, 'd> {
        let fun = Ptr::new(self.el);
        match idx.into() {
            IndexCatInt::Idx(idx) => {
                let idx = Ptr::new(idx.el);
                self.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<I0>();
                    fun.at::<V, I0>(idx)
                })
            }
            IndexCatInt::ConUsize(con) => self.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, I0>(con)
            }),
        }
    }
}
