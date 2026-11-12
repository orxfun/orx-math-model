use crate::sets::{SetSlot, SetTyp};
use crate::{new_bag, Bag, ValSymSum};
use alloc::string::String;
use core::marker::PhantomData;
use orx_imp_vec::Elem;

#[derive(derive_more::From)]
pub enum ArbVal<'d> {
    String(String),
    ValSymSum(ValSymSum<'d, 'd>),
}

impl<'d> ArbVal<'d> {
    pub fn as_string_ptr(&self) -> *const String {
        match self {
            ArbVal::String(x) => x,
            _ => unreachable!(),
        }
    }

    pub fn as_val_sym_sum(&self) -> &ValSymSum<'d, 'd> {
        match self {
            ArbVal::ValSymSum(x) => x,
            _ => unreachable!(),
        }
    }
}

pub struct ArbValBag<'d> {
    bag: Bag<ArbVal<'d>>,
    p: PhantomData<fn() -> &'d ()>,
}

impl Default for ArbValBag<'_> {
    fn default() -> Self {
        Self {
            bag: new_bag(),
            p: PhantomData,
        }
    }
}

impl<'d> ArbValBag<'d> {
    pub fn push<'m>(&'m self, data: impl Into<ArbVal<'d>>) -> Elem<'m, ArbVal<'d>> {
        self.bag.push(data.into())
    }
}
