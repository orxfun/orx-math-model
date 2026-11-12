use crate::sets::{SetSlot, SetTyp};
use crate::{new_bag, Bag};
use alloc::string::String;
use core::marker::PhantomData;
use orx_imp_vec::Elem;

#[derive(derive_more::From)]
pub enum ArbVal {
    String(String),
}

pub struct ArbValBag<'d> {
    bag: Bag<ArbVal>,
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
    pub fn push<'m>(&'m self, data: impl Into<ArbVal>) -> Elem<'m, ArbVal> {
        self.bag.push(data.into())
    }
}
