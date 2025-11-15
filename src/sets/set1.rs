use crate::{model::SetData, symbols::SymbolRef};
use core::marker::PhantomData;

pub struct Set1<'m, T1> {
    symbol: SymbolRef<'m, SetData>,
    p: PhantomData<T1>,
}

impl<'m, T1> Set1<'m, T1> {
    pub(crate) fn new(symbol: SymbolRef<'m, SetData>) -> Self {
        Self {
            symbol,
            p: PhantomData,
        }
    }
}

impl<'m, T1> Clone for Set1<'m, T1> {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol,
            p: PhantomData,
        }
    }
}

impl<'m, T1> Copy for Set1<'m, T1> {}
