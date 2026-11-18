use crate::vars::DVars;
use crate::{Model, Ptr, Set, Symbol};
use core::marker::PhantomData;
use core::ops::Index;

// d1

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD1<'m, 'd, I0: Set>(Symbol<'m, 'd, DVars<'d>>, PhantomData<I0>);

impl<'m, 'd, I0: Set> VarD1<'m, 'd, I0> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DVars<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn data_ptr(&self) -> Ptr<'d, DVars<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    // pub fn at(self, i0: impl Into<I0::IdxAny<'m, 'd>>) -> Var<'m, 'd> {
    //     I0::v1_at(self, i0)
    // }
}

// impl<'m, 'd, I0, Idx0> Index<Idx0> for VarD1<'m, 'd, I0>
// where
//     I0: Set,
//     Idx0: Into<I0::IdxAny<'m, 'd>>,
// {
//     type Output = Var<'m, 'd>;

//     fn index(&self, i0: Idx0) -> &Self::Output {
//         self.m().sym_ref::<ScaVar>(self.at(i0))
//     }
// }

// d2

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD2<'m, 'd, I0: Set, I1: Set>(Symbol<'m, 'd, DVars<'d>>, PhantomData<(I0, I1)>);

impl<'m, 'd, I0: Set, I1: Set> VarD2<'m, 'd, I0, I1> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DVars<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn data_ptr(&self) -> Ptr<'d, DVars<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    // pub fn at(
    //     self,
    //     i0: impl Into<I0::IdxAny<'m, 'd>>,
    //     i1: impl Into<I1::IdxAny<'m, 'd>>,
    // ) -> Var<'m, 'd> {
    //     I0::v2_at(self, i0, i1)
    // }
}

// impl<'m, 'd, I0, I1, Idx0, Idx1> Index<(Idx0, Idx1)> for VarD2<'m, 'd, I0, I1>
// where
//     I0: Set,
//     I1: Set,
//     Idx0: Into<I0::IdxAny<'m, 'd>>,
//     Idx1: Into<I1::IdxAny<'m, 'd>>,
// {
//     type Output = Var<'m, 'd>;

//     fn index(&self, (i0, i1): (Idx0, Idx1)) -> &Self::Output {
//         self.m().sym_ref::<ScaVar>(self.at(i0, i1))
//     }
// }
