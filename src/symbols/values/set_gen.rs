use alloc::boxed::Box;

pub trait SetGen<const N: usize> {
    fn elements(&self, indices: [usize; N]) -> Box<dyn Iterator<Item = usize> + '_>;
}

pub struct EmptySetGen<const N: usize>;

impl<const N: usize> SetGen<N> for EmptySetGen<N> {
    fn elements(&self, _: [usize; N]) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(core::iter::empty())
    }
}

pub struct FunSet<const N: usize, Data, F, I>
where
    I: IntoIterator<Item = usize>,
    F: Fn(&Data, [usize; N]) -> I,
{
    data: Data,
    fun: F,
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
