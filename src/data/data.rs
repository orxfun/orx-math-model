use crate::symbols::SetCoreMap;
use crate::SetAndData;
use alloc::boxed::Box;

pub struct Data<'m> {
    sets: SetCoreMap<'m, Box<dyn SetAndData<'m> + 'm>>,
}

impl<'m> Data<'m> {
    pub(crate) fn new(sets: SetCoreMap<'m, Box<dyn SetAndData<'m> + 'm>>) -> Self {
        Self { sets }
    }
}
