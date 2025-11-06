use crate::data::set_data::set_gen::SetGen;
use alloc::boxed::Box;

pub struct FunSetD1<'d, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    data: &'d Data,
    fun: fn(&'d Data, usize) -> I,
}

impl<'d, Data, I> FunSetD1<'d, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    pub fn new(data: &'d Data, fun: fn(&'d Data, usize) -> I) -> Self {
        Self { data, fun }
    }
}

impl<'d, Data, I> SetGen<1> for FunSetD1<'d, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    fn elements(&self, [i]: [usize; 1]) -> Box<dyn Iterator<Item = usize> + '_> {
        let elements = (self.fun)(self.data, i).into_iter();
        Box::new(elements)
    }
}
