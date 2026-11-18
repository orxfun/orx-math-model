use core::fmt::{Debug, Display};

struct Abc {}

impl Debug for Abc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Abc").finish()
    }
}

impl Display for Abc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
