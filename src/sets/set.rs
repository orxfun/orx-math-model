use crate::{pos::Pos, scalar_int::Int, symbol::Symbol, Model};

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'a>(pub(crate) Symbol<'a, SetData>);

// data

pub struct SetData;
