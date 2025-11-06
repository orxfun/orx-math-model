use crate::symbols::values::SetGen;
use alloc::boxed::Box;

pub struct FunSet<const N: usize, Data, F, I>
where
    I: IntoIterator<Item = usize>,
    F: Fn(&Data, [usize; N]) -> I,
{
    data: Data,
    fun: F,
}

impl<const N: usize, Data, F, I> FunSet<N, Data, F, I>
where
    I: IntoIterator<Item = usize>,
    F: Fn(&Data, [usize; N]) -> I,
{
    pub fn new(data: Data, fun: F) -> Self {
        Self { data, fun }
    }
}

impl<const N: usize, Data, F, I> SetGen<N> for FunSet<N, Data, F, I>
where
    I: IntoIterator<Item = usize>,
    F: Fn(&Data, [usize; N]) -> I,
{
    fn elements(&self, indices: [usize; N]) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new((self.fun)(&self.data, indices).into_iter())
    }
}
