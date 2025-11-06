use crate::symbols::values::set_gen::SetGen;
use alloc::boxed::Box;

pub enum Elements {
    D0(Box<dyn SetGen<0>>),
    D1(Box<dyn SetGen<1>>),
}

impl<S0: SetGen<0> + 'static> From<S0> for Elements {
    fn from(value: S0) -> Self {
        Self::D0(Box::new(value))
    }
}
