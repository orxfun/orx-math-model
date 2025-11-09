use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::data::set_data::set_and_data::SetAndDataCore;
use crate::data::set_data::set_gen::SetGen;
use crate::data::SetAndData;
use crate::symbols::sets::SetCore;
use crate::Set;
use alloc::boxed::Box;
use orx_self_or::SoR;

pub struct FunSetD0<'d, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    data: &'d Data,
    fun: fn(&'d Data) -> I,
}

impl<'d, 'm, Data, I, T> SetGen<'m> for FunSetD0<'d, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    fn elements(
        &self,
        _: SetCore<'m>,
        _: &SetDepths<'m>,
        _: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let elements = (self.fun)(self.data).into_iter().map(|x| *x.get_ref());
        Box::new(elements)
    }
}

// set & data

pub struct FunSetAndDataD0<'m, 'd, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    set: Set<'m, 0>,
    data: FunSetD0<'d, Data, I, T>,
}

impl<'m, 'd, Data, I, T> FunSetAndDataD0<'m, 'd, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    pub fn new(set: Set<'m, 0>, data: &'d Data, fun: fn(&'d Data) -> I) -> Self {
        Self {
            set,
            data: FunSetD0 { data, fun },
        }
    }
}

impl<'m, 'd, Data, I, T> SetAndData<'m, 0> for FunSetAndDataD0<'m, 'd, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    fn set(&self) -> Set<'m, 0> {
        self.set
    }

    fn elements(
        &'m self,
        set_depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let set = self.set.into();
        self.data.elements(set, set_depths, index_values)
    }
}

impl<'m, 'd, Data, I, T> SetAndDataCore<'m> for FunSetAndDataD0<'m, 'd, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    fn elements(
        &'m self,
        set_depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let set = self.set.into();
        self.data.elements(set, set_depths, index_values)
    }
}
