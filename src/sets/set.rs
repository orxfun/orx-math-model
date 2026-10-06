use crate::{pos::Pos, scalar_int::Int, symbol::Symbol, Model};

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'a>(pub(crate) Symbol<'a, SetData>);

// impl<'a> Set<'a> {
//     pub fn element(self) -> Int<'a> {
//         self.0.m.range_elem(self)
//     }
// }

// data

pub struct SetData;
