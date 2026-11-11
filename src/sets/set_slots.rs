use crate::{Bag, Elements, Slot};
use alloc::{boxed::Box, string::String};

pub enum SetSlot<'d> {
    CatSetInt(Slot<'d, Box<dyn Fn(&mut Elements<usize>) + 'd>>),
    CatSetStr(Slot<'d, Box<dyn Fn(&mut Elements<&'d String>) + 'd>>),
    Ord(Slot<'d, Box<dyn Fn(&mut Elements<i64>) + 'd>>),
}

pub struct SetSlots<'d> {
    cat_set_int: Bag<Box<dyn Fn(&mut Elements<usize>) + 'd>>,
    cat_set_str: Bag<Box<dyn Fn(&mut Elements<&'d String>) + 'd>>,
    ord_set: Bag<Box<dyn Fn(&mut Elements<i64>) + 'd>>,
}

impl<'d> SetSlots<'d> {
    pub fn add_cat_set_int(&self) -> SetSlot<'d> {
        let el = self.cat_set_int.push(Box::new(|_| {}));
        SetSlot::CatSetInt(Slot::from(el))
    }
}
