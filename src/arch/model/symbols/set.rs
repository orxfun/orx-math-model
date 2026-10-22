use crate::model::Set0Gen;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct Set<'m, 'd> {
    slot: Elem<'m, Set0Gen<'d>>,
}

impl<'d> Set<'_, 'd> {
    pub(crate) fn value(&self) -> &Set0Gen<'d> {
        // TODO: safety notes: we shouldn't send out the data ref to another thread
        unsafe { self.slot.get() }
    }

    pub(crate) fn set(&self, value: Set0Gen<'d>) {
        self.slot.set(value);
    }
}
