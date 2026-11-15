use crate::sets::DSet;
use crate::vars::VarType;
use crate::Ptr;

#[derive(derive_new::new)]
pub struct DV2<'d> {
    pub typ: VarType,
    pub s0: Ptr<'d, DSet<'d>>,
    pub s1: Ptr<'d, DSet<'d>>,
}
