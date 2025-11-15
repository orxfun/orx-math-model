use crate::{sets::SetSymData, symbols::SymbolRef};
use core::marker::PhantomData;

pub struct Set2<'m, T1, T2> {
    symbol: SymbolRef<'m, SetSymData>,
    p: PhantomData<(T1, T2)>,
}

impl<'m, T1, T2> Set2<'m, T1, T2> {
    pub(crate) fn new(symbol: SymbolRef<'m, SetSymData>) -> Self {
        Self {
            symbol,
            p: PhantomData,
        }
    }
}

impl<'m, T1, T2> Clone for Set2<'m, T1, T2> {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol,
            p: PhantomData,
        }
    }
}

impl<'m, T1, T2> Copy for Set2<'m, T1, T2> {}
