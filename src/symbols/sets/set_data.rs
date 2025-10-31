use crate::symbols::sets::set_gen::SetGen;
use alloc::boxed::Box;

pub struct SetData {
    pub kind: SetKind,
}

pub enum SetKind {
    Independent(Box<dyn SetGen>),
}
