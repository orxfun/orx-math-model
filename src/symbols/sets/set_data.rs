use crate::stages::Stage;
use crate::symbols::{sets::dep_set_indices::DependentSetIndices, Set};
use crate::Modeling;
use core::fmt::Debug;

#[derive(Default)]
pub struct SetData<S>
where
    S: Stage,
{
    depends_on: DependentSetIndices,
    stage_data: S::SetData,
}

impl SetData<Modeling> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Stage> SetData<S> {
    pub fn add_depending_set(&mut self, set: Set<'_>) {
        self.depends_on.push(set);
        for set in set.dependant_sets() {
            self.depends_on.push(set);
        }
    }

    // pub fn new(depends_on: impl IntoIterator<Item = usize>) -> Self {
    //     let mut depends_on = DependentSetIndices::default();
    //     depends_on.into_iter().for_each(|idx| depends_on.push(idx));
    //     let mut depends_on: Vec<_> = depends_on.into_iter().collect();
    //     depends_on.sort();
    //     Self { depends_on }
    // }

    pub fn depends_on_indices(&self) -> &[usize] {
        self.depends_on.indices()
    }
}

impl<S: Stage> Debug for SetData<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SetData")
            .field("depends_on", &self.depends_on_indices())
            .finish()
    }
}
