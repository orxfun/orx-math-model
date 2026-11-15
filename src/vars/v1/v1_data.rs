use crate::sets::DSet;
use crate::vars::VarType;
use crate::Ptr;

#[derive(derive_new::new)]
pub struct DV1<'d> {
    pub typ: VarType,
    pub s0: Ptr<'d, DSet<'d>>,
}
