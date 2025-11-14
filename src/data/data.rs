use crate::data::par_data::ParData;
use crate::symbols::{ParCoreMap, SetCoreMap};
use crate::{Model, SetDataCore};
use alloc::boxed::Box;

// TODO: temporary allow
#[allow(dead_code)]
pub struct Data<'m> {
    model: &'m Model,
    sets: SetCoreMap<'m, Box<dyn SetDataCore<'m> + 'm>>,
    pars: ParCoreMap<'m, Box<dyn ParData<'m> + 'm>>,
}

impl<'m> Data<'m> {
    pub(crate) fn new(
        model: &'m Model,
        sets: SetCoreMap<'m, Box<dyn SetDataCore<'m> + 'm>>,
        pars: ParCoreMap<'m, Box<dyn ParData<'m> + 'm>>,
    ) -> Self {
        Self { model, sets, pars }
    }
}
