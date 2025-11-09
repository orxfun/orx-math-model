use crate::data::number::Number;
use core::marker::PhantomData;
use orx_v::V1;

pub struct FunParD1<N, D>
where
    N: Number,
    D: V1<N>,
{
    data: D,
    phantom: PhantomData<N>,
}
