use alloc::boxed::Box;

pub trait SetGen<const N: usize> {
    fn elements(&self, indices: [usize; N]) -> Box<dyn Iterator<Item = usize> + '_>;
}
