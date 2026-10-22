use crate::sets::{Element, Set1};
use core::marker::PhantomData;

pub struct DepSet1<'m, D1, T1>
where
    D1: Element,
    T1: Element,
{
    dep_set: Set1<'m, D1>,
    p: PhantomData<T1>,
}
