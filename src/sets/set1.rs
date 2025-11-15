use crate::sets::Set2;
use crate::sets::{element::Element, SetSymData};
use crate::symbols::SymbolRef;
use alloc::string::{String, ToString};
use core::ops::Mul;
use core::{fmt::Display, marker::PhantomData};

pub struct Set1<'m, T1>
where
    T1: Element,
{
    symbol: SymbolRef<'m, SetSymData>,
    p: PhantomData<T1>,
}

impl<'m, T1> Set1<'m, T1>
where
    T1: Element,
{
    pub(crate) fn new(symbol: SymbolRef<'m, SetSymData>) -> Self {
        Self {
            symbol,
            p: PhantomData,
        }
    }
}

impl<'m, T1> Clone for Set1<'m, T1>
where
    T1: Element,
{
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol,
            p: PhantomData,
        }
    }
}

impl<'m, T1> Copy for Set1<'m, T1> where T1: Element {}

impl<'m, T1> Display for Set1<'m, T1>
where
    T1: Element,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let key = &self.symbol.data_ref.key;
        let key = key.map_or_else(|| "UNNAMED_SET".to_string(), |x| x.clone());
        write!(f, "{}", key)
    }
}

impl<'m, T1> Set1<'m, T1>
where
    T1: Element,
{
    pub fn key(self, value: impl Into<String>) -> Self {
        self.symbol.data_ref.set_key(value);
        self
    }

    pub fn definition(self, value: impl Into<String>) -> Self {
        self.symbol.data_ref.set_definition(value);
        self
    }
}

// ops

// TODO!

impl<'m, T1, T2> Mul<Set1<'m, T2>> for Set1<'m, T1>
where
    T1: Element,
    T2: Element,
{
    type Output = Set2<'m, T1, T2>;

    fn mul(self, rhs: Set1<'m, T2>) -> Self::Output {
        todo!()
    }
}
