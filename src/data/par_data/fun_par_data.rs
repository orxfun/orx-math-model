use crate::{
    data::{number::Number, IndexValuesIter},
    symbols::pars::ParCore,
};
use core::marker::PhantomData;
use orx_self_or::SoR;

pub struct FunParData<'d, 'm, Data, N, T, F>
where
    N: Number,
    T: SoR<N>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> T,
{
    par: ParCore<'m>,
    data: &'d Data,
    fun: F,
    phantom: PhantomData<N>,
}
