use crate::draft::{scalar::Scalar, set::*};
use core::{marker::PhantomData, ops::Index};

pub struct ParRef<P: Par>(PhantomData<P>);
impl<P: Par> ParRef<P> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<P: Par> Clone for ParRef<P> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P: Par> Copy for ParRef<P> {}

pub trait Par: Clone + Copy {
    type Value;
}

pub trait Par0: Par {}

pub trait Par1: Par {
    type S0: Set0;

    fn value(&self, element: &<Self::S0 as Set>::Elem) -> Self::Value;
}

impl<P: Par1> Index<P::S0> for ParRef<P> {
    type Output = Scalar;
    fn index(&self, _: P::S0) -> &Self::Output {
        &Scalar
    }
}
