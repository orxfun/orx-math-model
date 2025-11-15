use crate::sets::element::Element;
use crate::sets::set1_data::Set1Data;
use crate::sets::Set1;

pub struct Subset1<'m, T1, F>
where
    T1: Element,
    F: Fn(&T1) -> bool,
{
    set: Set1<'m, T1>,
    filter: F,
}

impl<'m, T1, F> Subset1<'m, T1, F>
where
    T1: Element,
    F: Fn(&T1) -> bool,
{
    pub(crate) fn new(set: Set1<'m, T1>, filter: F) -> Self {
        Self { set, filter }
    }

    pub(crate) fn values<D, G, I>(
        &'m self,
        set_data: &Set1Data<'m, D, T1, G, I>,
    ) -> impl Iterator<Item = &'m T1>
    where
        I: IntoIterator<Item = &'m T1>,
        G: Fn(&'m D) -> I,
    {
        set_data.values().into_iter().filter(|x| (self.filter)(x))
    }
}
