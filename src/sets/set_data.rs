use crate::{FunSlot, SetTyp};

pub struct DSet<'d> {
    typ: SetTyp,
    fun: FunSlot<'d>,
}
