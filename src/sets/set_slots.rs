use crate::{new_bag, Bag, Elements, Slot};
use alloc::{boxed::Box, string::String};

pub enum SetType {
    CatSetInt,
    CatSetStr,
    OrdSet,
}

pub enum SetSlot<'d> {
    CatSetInt(Slot<'d, Box<dyn Fn(&mut Elements<usize>) + 'd>>),
    CatSetStr(Slot<'d, Box<dyn Fn(&mut Elements<&'d String>) + 'd>>),
    OrdSet(Slot<'d, Box<dyn Fn(&mut Elements<i64>) + 'd>>),
}

pub struct SetSlots<'d> {
    cat_set_int: Bag<Box<dyn Fn(&mut Elements<usize>) + 'd>>,
    cat_set_str: Bag<Box<dyn Fn(&mut Elements<&'d String>) + 'd>>,
    ord_set: Bag<Box<dyn Fn(&mut Elements<i64>) + 'd>>,
}

impl Default for SetSlots<'_> {
    fn default() -> Self {
        Self {
            cat_set_int: new_bag(),
            cat_set_str: new_bag(),
            ord_set: new_bag(),
        }
    }
}

impl<'d> SetSlots<'d> {
    pub fn push(&self, t: SetType) -> SetSlot<'d> {
        match t {
            SetType::CatSetInt => {
                let el = self.cat_set_int.push(Box::new(|_| {}));
                SetSlot::CatSetInt(Slot::from(el))
            }
            SetType::CatSetStr => {
                let el = self.cat_set_str.push(Box::new(|_| {}));
                SetSlot::CatSetStr(Slot::from(el))
            }
            SetType::OrdSet => {
                let el = self.ord_set.push(Box::new(|_| {}));
                SetSlot::OrdSet(Slot::from(el))
            }
        }
    }
}
