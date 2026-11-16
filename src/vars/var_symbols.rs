use crate::vars::DV;
use crate::{ScaVar, Set, Symbol, Var};
use core::ops::Index;
use core::{marker::PhantomData, ops::Deref};

// d1

#[derive(derive_more::From, Clone, Copy)]
pub struct VarD1<'m, 'd, I0: Set>(Symbol<'m, 'd, DV<'d>>, PhantomData<I0>);

// d2

#[derive(derive_more::From, Clone, Copy)]
pub struct VarD2<'m, 'd, I0: Set, I1: Set>(Symbol<'m, 'd, DV<'d>>, PhantomData<(I0, I1)>);
