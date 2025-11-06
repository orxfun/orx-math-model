use crate::symbols::values::set_gen::{EmptySetGen, SetGen};
use alloc::boxed::Box;

pub enum Elements {
    D0(Box<dyn SetGen<0>>),
    D1(Box<dyn SetGen<1>>),
    D2(Box<dyn SetGen<2>>),
}

impl Elements {
    pub fn empty(dim: usize) -> Self {
        match dim {
            0 => Self::D0(Box::new(EmptySetGen::<0>)),
            1 => Self::D1(Box::new(EmptySetGen::<1>)),
            2 => Self::D2(Box::new(EmptySetGen::<2>)),
            _ => todo!(),
        }
    }

    pub fn dim(&self) -> usize {
        match self {
            Self::D0(_) => 0,
            Self::D1(_) => 1,
            Self::D2(_) => 2,
        }
    }
}
