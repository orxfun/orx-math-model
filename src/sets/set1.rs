use core::marker::PhantomData;

pub struct Set1<'m, T1> {
    p: PhantomData<&'m T1>,
}

impl<'m, T1> Default for Set1<'m, T1> {
    fn default() -> Self {
        Self { p: PhantomData }
    }
}

impl<'m, T1> Clone for Set1<'m, T1> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<'m, T1> Copy for Set1<'m, T1> {}
