use crate::constants::MAX_DIM;
use crate::idxs::DIdx;
use crate::{ConVec, Ptr};

#[derive(Clone)]
pub struct IdxVec<'d>(ConVec<MAX_DIM, Ptr<'d, DIdx<'d>>>);
