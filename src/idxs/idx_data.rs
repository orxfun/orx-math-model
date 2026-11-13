use crate::idxs::IdxSlot;
use crate::sets::DSet;
use crate::Ptr;

#[derive(derive_new::new)]
pub struct DIdx<'d> {
    pub set: Ptr<'d, DSet<'d>>,
    pub idx: IdxSlot<'d>,
}
