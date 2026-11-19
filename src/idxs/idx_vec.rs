use crate::idxs::DIdx;
use crate::{ConVec, Ptr};

#[derive(Clone)]
pub struct IdxVec<'d>(ConVec<8, Ptr<'d, DIdx<'d>>>);
