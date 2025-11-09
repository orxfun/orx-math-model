use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::data::set_data::{set_and_data::SetAndData, set_gen::SetGen};
use crate::Set;
use alloc::boxed::Box;
use orx_self_or::SoR;

pub struct FunSetD1<'d, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    data: &'d Data,
    fun: fn(&'d Data, usize) -> I,
}

impl<'d, Data, I, T> SetGen<1> for FunSetD1<'d, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    fn elements_by_dependencies(&self, [i]: [usize; 1]) -> Box<dyn Iterator<Item = usize> + '_> {
        let elements = (self.fun)(self.data, i).into_iter().map(|x| *x.get_ref());
        Box::new(elements)
    }
}

// set & data

pub struct FunSetAndDataD1<'m, 'd, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    set: Set<'m, 1>,
    data: FunSetD1<'d, Data, I, T>,
}

impl<'m, 'd, Data, I, T> FunSetAndDataD1<'m, 'd, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    pub fn new(set: Set<'m, 1>, data: &'d Data, fun: fn(&'d Data, usize) -> I) -> Self {
        Self {
            set,
            data: FunSetD1 { data, fun },
        }
    }
}

impl<'m, 'd, Data, I, T> SetAndData<'m, 1> for FunSetAndDataD1<'m, 'd, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    fn set(&self) -> Set<'m, 1> {
        self.set
    }

    fn elements(
        &'m self,
        set_depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        self.data.elements(self.set(), set_depths, index_values)
    }
}
