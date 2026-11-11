use crate::sets::{SetBag, SetSlots};

#[derive(Default)]
pub struct Model<'d> {
    // sets
    set_slots: SetSlots<'d>,
    set_bag: SetBag<'d>,
}

impl<'d> Model<'d> {
    // sets

    pub fn cat_set_int(&self) {}
}
