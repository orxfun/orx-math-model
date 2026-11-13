use crate::{SetTyp, Slot};

pub struct DSet<'d> {
    typ: SetTyp,
    fun: Slot<'d, usize>,
}

// pub enum SetSlot<'d> {
//     CatSetInt(Slot<'d, Box<dyn Fn(&mut Elements<usize>) + 'd>>),
//     CatSetStr(Slot<'d, Box<dyn Fn(&mut Elements<&'d String>) + 'd>>),
//     OrdSet(Slot<'d, Box<dyn Fn(&mut Elements<i64>) + 'd>>),
// }
