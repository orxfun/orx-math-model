use crate::scalar_int::{FunIntData, RangeElemData};
use crate::symbol::Symbol;
use crate::{pos::Pos, Model};

#[derive(Clone, Copy)]
pub struct Cat<'a>(pub(crate) Symbol<'a, CatData>);

// data

pub struct CatData(Pos);
