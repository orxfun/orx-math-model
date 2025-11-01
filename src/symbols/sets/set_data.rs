use core::fmt::Debug;

pub enum SetKind {
    Independent,
}

impl Debug for SetKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Independent => f.debug_tuple("Independent").finish(),
        }
    }
}
