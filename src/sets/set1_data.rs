use crate::sets::{Element, Set1};

pub struct Set1Data<'m, D, T1, F, I>
where
    T1: Element + 'm,
    I: IntoIterator<Item = &'m T1>,
    F: Fn(&'m D) -> I,
{
    data: &'m D,
    set1: Set1<'m, T1>,
    fun: F,
}

impl<'m, D, T1, F, I> Set1Data<'m, D, T1, F, I>
where
    T1: Element + 'm,
    I: IntoIterator<Item = &'m T1>,
    F: Fn(&'m D) -> I,
{
    pub(crate) fn new(data: &'m D, set1: Set1<'m, T1>, fun: F) -> Self {
        Self { data, set1, fun }
    }

    pub(crate) fn values(&self) -> I {
        (self.fun)(&self.data)
    }
}
