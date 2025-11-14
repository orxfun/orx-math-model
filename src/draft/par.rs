use crate::draft::{scalar::Scalar, set::*};
use core::{marker::PhantomData, ops::Index};

pub struct ParRef<P: Par>(PhantomData<P>);
impl<P: Par> ParRef<P> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub trait Par {
    type Value;
}

pub trait Par0: Par {}

pub trait Par1: Par {
    type S0: Set0;

    fn value(&self, element: &<Self::S0 as Set>::Elem) -> Self::Value;
}

impl<P: Par1> Index<<P::S0 as Set>::Key> for ParRef<P> {
    type Output = Scalar;
    fn index(&self, _: <P::S0 as Set>::Key) -> &Self::Output {
        &Scalar
    }
}
