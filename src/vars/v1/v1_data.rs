use crate::sets::DSet;
use crate::vars::VarType;
use crate::{Elements, Ptr, Res, ResValid, Set, SetTyp, Slot, Val};

#[derive(derive_new::new)]
pub struct DV1<'d> {
    s0: Ptr<'d, DSet<'d>>,
    typ: VarType,
}
