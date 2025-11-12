use crate::data::par_data::ParAndData;
use crate::symbols::{ParCoreMap, SetCoreMap};
use crate::{Model, SetAndData};
use alloc::boxed::Box;

// TODO: temporary allow
#[allow(dead_code)]
pub struct Data<'m> {
    model: &'m Model,
    sets: SetCoreMap<'m, Box<dyn SetAndData<'m> + 'm>>,
    pars: ParCoreMap<'m, Box<dyn ParAndData<'m> + 'm>>,
}

impl<'m> Data<'m> {
    pub(crate) fn new(
        model: &'m Model,
        sets: SetCoreMap<'m, Box<dyn SetAndData<'m> + 'm>>,
        pars: ParCoreMap<'m, Box<dyn ParAndData<'m> + 'm>>,
    ) -> Self {
        Self { model, sets, pars }
    }
}
