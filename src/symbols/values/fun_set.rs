use crate::symbols::values::SetGen;
use alloc::boxed::Box;
use core::marker::PhantomData;

pub struct FunSet<'a, const N: usize, Data, F, I>
where
    I: IntoIterator<Item = usize> + 'a,
    F: Fn(&'a Data, [usize; N]) -> I,
    Data: 'a,
{
    data: Data,
    fun: F,
    phantom: PhantomData<&'a ()>,
}

impl<'a, const N: usize, Data, F, I> FunSet<'a, N, Data, F, I>
where
    I: IntoIterator<Item = usize> + 'a,
    F: Fn(&'a Data, [usize; N]) -> I,
{
    pub fn new(data: Data, fun: F) -> Self {
        Self {
            data,
            fun,
            phantom: PhantomData,
        }
    }
}

// impl<'a, const N: usize, Data, F, I> SetGen<N> for FunSet<'a, N, Data, F, I>
// where
//     I: IntoIterator<Item = usize>,
//     F: Fn(&Data, [usize; N]) -> I,
// {
//     fn elements(&self, indices: [usize; N]) -> Box<dyn Iterator<Item = usize> + '_> {
//         Box::new((self.fun)(&self.data, indices).into_iter())
//     }
// }

impl<I> SetGen<1> for Box<dyn Fn(usize) -> I>
where
    I: Iterator<Item = usize>,
{
    fn elements(&self, [i]: [usize; 1]) -> Box<dyn Iterator<Item = usize> + '_> {
        let x = self(i);
        Box::new(x)
    }
}

impl<I> SetGen<2> for Box<dyn Fn(usize, usize) -> I>
where
    I: Iterator<Item = usize>,
{
    fn elements(&self, [i, j]: [usize; 2]) -> Box<dyn Iterator<Item = usize> + '_> {
        let x = self(i, j);
        Box::new(x)
    }
}
