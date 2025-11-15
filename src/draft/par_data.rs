use crate::draft::par::*;
use crate::draft::set::*;

pub trait ParData0 {
    type Par: Par0;

    fn value(&self) -> <Self::Par as Par>::Value;
}

pub trait ParData1 {
    type Par: Par1;

    fn value(&self, element: &<<Self::Par as Par1>::S0 as Set>::Key) -> <Self::Par as Par>::Value;
}
