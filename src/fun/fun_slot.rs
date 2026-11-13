use crate::Slot;
use alloc::boxed::Box;

#[derive(derive_new::new)]
pub struct FunSlot<'d> {
    slot: Slot<'d, Box<dyn Fn() + 'd>>,
}
