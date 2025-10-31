use crate::symbols::sets::set_gen::SetGen;
use alloc::boxed::Box;
use core::fmt::Debug;

pub struct SetData {
    pub kind: SetKind,
}

pub enum SetKind {
    Independent(Box<dyn SetGen>),
}

impl Debug for SetKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Independent(_) => f.debug_tuple("Independent").finish(),
        }
    }
}
