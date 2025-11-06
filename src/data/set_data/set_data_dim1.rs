use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::data::set_data::{set_and_data::SetAndData, set_gen::SetGen};
use crate::{symbols::SetCore, Set};
use alloc::boxed::Box;

pub struct FunSetD1<'d, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    data: &'d Data,
    fun: fn(&'d Data, usize) -> I,
}

impl<'d, Data, I> SetGen<1> for FunSetD1<'d, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    fn elements_by_dependencies(&self, [i]: [usize; 1]) -> Box<dyn Iterator<Item = usize> + '_> {
        let elements = (self.fun)(self.data, i).into_iter();
        Box::new(elements)
    }

    fn elements<'m>(
        &self,
        set: SetCore<'m>,
        depths: SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        set.sym_data().depends_on_indices();
        todo!()
    }
}

// set & data

pub struct FunSetAndDataD1<'m, 'd, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    set: Set<'m, 1>,
    data: FunSetD1<'d, Data, I>,
}

impl<'m, 'd, Data, I> FunSetAndDataD1<'m, 'd, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    pub fn new(set: Set<'m, 1>, data: &'d Data, fun: fn(&'d Data, usize) -> I) -> Self {
        Self {
            set,
            data: FunSetD1 { data, fun },
        }
    }
}

impl<'m, 'd, Data, I> SetAndData<'m, 1> for FunSetAndDataD1<'m, 'd, Data, I>
where
    I: IntoIterator<Item = usize>,
{
    fn set(&self) -> Set<'m, 1> {
        self.set
    }

    fn data(&self) -> &impl SetGen<1> {
        &self.data
    }
}
