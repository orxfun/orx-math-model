use crate::sets::DSet;
use crate::var::FunV;
use crate::vars::VarType;
use crate::{Dim, Elements, Ptr, Res, ResValid, Set, SetTyp, Slot, Val};

#[derive(derive_new::new)]
pub struct DV<'d> {
    typ: VarType,
    dim: Dim,
    fun: Ptr<'d, FunV<'d>>,
}
