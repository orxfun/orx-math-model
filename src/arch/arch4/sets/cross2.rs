use crate::sets::{set1_data::Set1Data, subset2::Subset2, Element, Set1};
use core::ops::BitOr;

pub struct CrossProdSet2<'m, T1, T2>
where
    T1: Element,
    T2: Element,
{
    set1: Set1<'m, T1>,
    set2: Set1<'m, T2>,
}

impl<'m, T1, T2> CrossProdSet2<'m, T1, T2>
where
    T1: Element,
    T2: Element,
{
    pub(crate) fn new(set1: Set1<'m, T1>, set2: Set1<'m, T2>) -> Self {
        Self { set1, set2 }
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
    }

    pub fn st<F>(&self, filter: F) -> Subset2<'m, T1, T2, F>
    where
        F: Fn(&T1, &T2) -> bool,
    {
        // TODO: we must add the subset1 to model and return a reference to it instead
        Subset2::new(self.set1, self.set2, filter)
    }
}

// ops

impl<'m, T1, T2, F> BitOr<F> for CrossProdSet2<'m, T1, T2>
where
    T1: Element,
    T2: Element,
    F: Fn(&T1, &T2) -> bool,
{
    type Output = Subset2<'m, T1, T2, F>;

    fn bitor(self, rhs: F) -> Self::Output {
        self.st(rhs)
    }
}
