use crate::SetAndData;
use alloc::boxed::Box;

pub struct Data<'m> {
    set0: Box<dyn SetAndData<'m, 0>>,
    set1: Box<dyn SetAndData<'m, 1>>,
}
