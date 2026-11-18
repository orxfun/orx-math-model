use crate::scalars::{FVarIdx, FVarWrite};
use crate::vars::DVars;
use crate::Ptr;

#[derive(derive_new::new)]
pub struct DVar<'d> {
    parent: Ptr<'d, DVars<'d>>,
    fun_idx: FVarIdx<'d>,
    fun_write: FVarWrite<'d>,
}
