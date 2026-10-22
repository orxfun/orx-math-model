use crate::data::par_data::ParDataCore;
use crate::data::SetDataCore;
use crate::symbols::{ParCoreMap, SetCoreMap};
use crate::Model;
use alloc::boxed::Box;

// TODO: temporary allow
#[allow(dead_code)]
pub struct Data<'m> {
    model: &'m Model,
    sets: SetCoreMap<'m, Box<dyn SetDataCore<'m> + 'm>>,
    pars: ParCoreMap<'m, Box<dyn ParDataCore<'m> + 'm>>,
}

impl<'m> Data<'m> {
    pub(crate) fn new(
        model: &'m Model,
        sets: SetCoreMap<'m, Box<dyn SetDataCore<'m> + 'm>>,
        pars: ParCoreMap<'m, Box<dyn ParDataCore<'m> + 'm>>,
    ) -> Self {
        Self { model, sets, pars }
    }
}
