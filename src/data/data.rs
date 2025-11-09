use crate::SetAndData;
use alloc::boxed::Box;

pub struct Data<'m> {
    set0: Box<dyn SetAndData<'m, 0>>,
}
