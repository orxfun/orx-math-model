use crate::{sets::SetSymData, symbols::SymbolRef};
use alloc::string::ToString;
use core::{fmt::Display, marker::PhantomData};

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

impl<'m, T1, T2> Display for Set2<'m, T1, T2> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let key = &self.symbol.data_ref.key;
        let key = key.map_or_else(|| "UNNAMED_SET".to_string(), |x| x.clone());
        writeln!(f, "{}", key)
    }
}
