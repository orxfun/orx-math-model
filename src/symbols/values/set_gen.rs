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
