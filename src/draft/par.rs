use crate::draft::set::*;

pub trait Par {
    type Value;
}

pub trait Par0: Par {}

pub trait Par1: Par {
    type S0: Set0;

    fn value(&self, element: &<Self::S0 as Set>::Elem) -> Self::Value;
}
