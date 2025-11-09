use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::data::set_data::set_and_data::{SetAndData, SetAndDataCore};
use crate::data::set_data::set_gen::SetGen;
use crate::symbols::sets::SetCore;
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

impl<'d, 'm, Data, I, T> SetGen<'m> for FunSetD1<'d, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    fn elements(
        &self,
        set: SetCore<'m>,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let mut dep_sets = set.depending_sets_core();
        // SAFETY: can have only 1 dep set since it can only be created with Set<1>.
        let s1 = dep_sets.next().unwrap();
        let depth_i1 = depths.depth_of(s1);
        let i1 = index_values[depth_i1];
        let elements = (self.fun)(self.data, i1).into_iter().map(|x| *x.get_ref());
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
        let set = self.set.into();
        self.data.elements(set, set_depths, index_values)
    }
}

impl<'m, 'd, Data, I, T> SetAndDataCore<'m> for FunSetAndDataD1<'m, 'd, Data, I, T>
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
