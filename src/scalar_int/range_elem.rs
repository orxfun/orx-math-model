use crate::{pos::Pos, symbol::Symbol, Model};

#[derive(Clone, Copy)]
pub struct RangeElem<'a>(pub(crate) Symbol<'a, RangeElemData>);

#[derive(derive_new::new)]
pub struct RangeElemData {
    pub range_pos: Pos,
    pub value_pos: Pos,
}
