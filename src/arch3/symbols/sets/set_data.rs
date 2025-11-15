use crate::symbols::dep_set_indices::DependentSetIndices;
use core::fmt::Debug;

pub struct SetData {
    depends_on: DependentSetIndices,
}

impl SetData {
    pub fn new(depends_on: DependentSetIndices) -> Self {
        Self { depends_on }
    }

    pub fn depends_on_indices(&self) -> &[usize] {
        self.depends_on.indices()
    }
}

impl Debug for SetData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("depends_on_indices")
            .field("depends_on", &self.depends_on_indices())
            .finish()
    }
}
