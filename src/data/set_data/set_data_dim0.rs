use crate::data::{set_data::set_gen::SetGen, SetAndData};
use crate::Set;
use alloc::boxed::Box;

pub struct FunSetD0<'d, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    data: &'d Data,
    fun: fn(&'d Data) -> I,
}

impl<'d, Data, I> SetGen<0> for FunSetD0<'d, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    fn elements(&self, _: [usize; 0]) -> Box<dyn Iterator<Item = usize> + '_> {
        let elements = (self.fun)(self.data).into_iter();
        Box::new(elements)
    }
}

// set & data

pub struct FunSetAndDataD0<'m, 'd, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    set: Set<'m, 0>,
    data: FunSetD0<'d, Data, I>,
}

impl<'m, 'd, Data, I> FunSetAndDataD0<'m, 'd, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    pub fn new(set: Set<'m, 0>, data: &'d Data, fun: fn(&'d Data) -> I) -> Self {
        Self {
            set,
            data: FunSetD0 { data, fun },
        }
    }
}

impl<'m, 'd, Data, I> SetAndData<'m, 0> for FunSetAndDataD0<'m, 'd, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    fn set(&self) -> Set<'m, 0> {
        self.set
    }

    fn data(&self) -> &impl SetGen<0> {
        &self.data
    }
}
