use crate::scalars::{FunVar1, FunVar2, UFunVar};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dim {
    D0,
    D1,
    D2,
}

// trait

pub trait D: Clone + Copy + 'static {
    fn dim() -> Dim;

    // var

    type FunVar<'d>;

    fn unpack_fun_var<'a, 'd>(u: &'a UFunVar<'d>) -> &'a Self::FunVar<'d>;
}

// d0

#[derive(Clone, Copy)]
pub struct D0;

impl D for D0 {
    fn dim() -> Dim {
        Dim::D0
    }

    // var

    type FunVar<'d> = usize;

    #[inline(always)]
    fn unpack_fun_var<'a, 'd>(u: &'a UFunVar<'d>) -> &'a Self::FunVar<'d> {
        todo!()
    }
}

// d1

#[derive(Clone, Copy)]
pub struct D1;

impl D for D1 {
    fn dim() -> Dim {
        Dim::D1
    }

    // var

    type FunVar<'d> = FunVar1<'d>;

    #[inline(always)]
    fn unpack_fun_var<'a, 'd>(u: &'a UFunVar<'d>) -> &'a Self::FunVar<'d> {
        unsafe { &u.d1 }
    }
}

// d2

#[derive(Clone, Copy)]
pub struct D2;

impl D for D2 {
    fn dim() -> Dim {
        Dim::D2
    }

    // var

    type FunVar<'d> = FunVar2<'d>;

    #[inline(always)]
    fn unpack_fun_var<'a, 'd>(u: &'a UFunVar<'d>) -> &'a Self::FunVar<'d> {
        unsafe { &u.d2 }
    }
}
