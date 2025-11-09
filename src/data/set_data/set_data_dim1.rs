use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::data::set_data::set_gen::SetGenCore;
use crate::data::set_data::{set_and_data::SetAndData, set_gen::SetGen};
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

impl<'d, Data, I, T> SetGenCore for FunSetD1<'d, Data, I, T>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
{
    fn elements2<'m>(
        &self,
        set: SetCore<'m>,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let dep_sets = set.depending_sets_core();
        let depths = dep_sets.map(|s| depths.depth_of(s));
        let mut depending_indices = depths.map(|d| index_values[d]);
        let i = depending_indices.next().unwrap();
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
        let set = self.set.into();
        self.data.elements2(set, set_depths, index_values)
    }
}
