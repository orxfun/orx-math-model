use crate::sets::element::Element;
use crate::sets::set1_data::Set1Data;
use crate::sets::Set1;

pub struct Subset2<'m, T1, T2, F>
where
    T1: Element,
    T2: Element,
    F: Fn(&T1, &T2) -> bool,
{
    set1: Set1<'m, T1>,
    set2: Set1<'m, T2>,
    filter: F,
}

impl<'m, T1, T2, F> Subset2<'m, T1, T2, F>
where
    T1: Element,
    T2: Element,
    F: Fn(&T1, &T2) -> bool,
{
    pub(crate) fn new(set1: Set1<'m, T1>, set2: Set1<'m, T2>, filter: F) -> Self {
        Self { set1, set2, filter }
    }

    pub(crate) fn values<D, G1, I1, G2, I2>(
        &'m self,
        set_data1: &'m Set1Data<'m, D, T1, G1, I1>,
        set_data2: &'m Set1Data<'m, D, T2, G2, I2>,
    ) -> impl Iterator<Item = (&'m T1, &'m T2)>
    where
        I1: IntoIterator<Item = &'m T1>,
        G1: Fn(&'m D) -> I1,
        I2: IntoIterator<Item = &'m T2>,
        G2: Fn(&'m D) -> I2,
    {
        // TODO: assert that set_data1.set is same as self.set
        set_data1
            .values()
            .into_iter()
            .flat_map(|x1| set_data2.values().into_iter().map(move |x2| (x1, x2)))
            .filter(|(x1, x2)| (self.filter)(x1, x2))
    }
}
