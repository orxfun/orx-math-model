use crate::symbols::DependentSetIndices;
use core::fmt::Debug;

pub struct ParData {
    depends_on: DependentSetIndices,
}

impl ParData {
    pub fn new(depends_on: DependentSetIndices) -> Self {
        Self { depends_on }
    }

    pub fn depends_on_indices(&self) -> &[usize] {
        self.depends_on.indices()
    }
}

impl Debug for ParData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SetData")
            .field("depends_on", &self.depends_on_indices())
            .finish()
    }
}
