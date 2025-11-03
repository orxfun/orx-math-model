use crate::symbols::sets::{dep_set_indices::DependentSetIndices, r#ref::SetCore};
use core::fmt::Debug;

#[derive(Default)]
pub struct SetData {
    depends_on: DependentSetIndices,
}

impl SetData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_depending_set(&mut self, set: SetCore<'_>) {
        // self.depends_on.push(set);
        // for set in set.dependant_sets() {
        //     self.depends_on.push(set);
        // }
        todo!()
    }

    pub fn depends_on_indices(&self) -> &[usize] {
        self.depends_on.indices()
    }
}

impl Debug for SetData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SetData")
            .field("depends_on", &self.depends_on_indices())
            .finish()
    }
}
