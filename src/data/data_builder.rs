use crate::symbols::sets::IndependentSetCollection;
use crate::symbols::SetCoreMap;
use crate::SetAndData;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Default)]
pub struct DataBuilder<'m> {
    set0: SetCoreMap<'m, Box<dyn SetAndData<'m, 0>>>,
    set1: SetCoreMap<'m, Box<dyn SetAndData<'m, 1>>>,
}

impl<'m> DataBuilder<'m> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sets(mut self, sets: impl IndependentSetCollection<'m>) -> Self {
        for set in sets.into_iter() {
            match set.dim() {
                0 => {}
                1 => {}
                _ => todo!(),
            }
        }
        self
    }
}
