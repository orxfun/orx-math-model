use core::marker::PhantomData;

pub struct Set2<'m, T1, T2> {
    p: PhantomData<&'m (T1, T2)>,
}

impl<'m, T1, T2> Default for Set2<'m, T1, T2> {
    fn default() -> Self {
        Self { p: PhantomData }
    }
}

impl<'m, T1, T2> Clone for Set2<'m, T1, T2> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<'m, T1, T2> Copy for Set2<'m, T1, T2> {}
