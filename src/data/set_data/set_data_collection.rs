use crate::SetData;
use alloc::boxed::Box;

pub trait SetDataCollection<'m> {
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetData<'m> + 'm>>;
}

impl<'m, S0> SetDataCollection<'m> for S0
where
    S0: SetData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetData<'m> + 'm>> {
        let s0: Box<dyn SetData<'m> + 'm> = Box::new(self);
        core::iter::once(s0)
    }
}

impl<'m, S0, S1> SetDataCollection<'m> for (S0, S1)
where
    S0: SetData<'m> + 'm,
    S1: SetData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetData<'m> + 'm>> {
        let s0: Box<dyn SetData<'m> + 'm> = Box::new(self.0);
        let s1: Box<dyn SetData<'m> + 'm> = Box::new(self.1);
        [s0, s1]
    }
}

impl<'m, S0, S1, S2> SetDataCollection<'m> for (S0, S1, S2)
where
    S0: SetData<'m> + 'm,
    S1: SetData<'m> + 'm,
    S2: SetData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetData<'m> + 'm>> {
        let s0: Box<dyn SetData<'m> + 'm> = Box::new(self.0);
        let s1: Box<dyn SetData<'m> + 'm> = Box::new(self.1);
        let s2: Box<dyn SetData<'m> + 'm> = Box::new(self.2);
        [s0, s1, s2]
    }
}

impl<'m, S0, S1, S2, S3> SetDataCollection<'m> for (S0, S1, S2, S3)
where
    S0: SetData<'m> + 'm,
    S1: SetData<'m> + 'm,
    S2: SetData<'m> + 'm,
    S3: SetData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetData<'m> + 'm>> {
        let s0: Box<dyn SetData<'m> + 'm> = Box::new(self.0);
        let s1: Box<dyn SetData<'m> + 'm> = Box::new(self.1);
        let s2: Box<dyn SetData<'m> + 'm> = Box::new(self.2);
        let s3: Box<dyn SetData<'m> + 'm> = Box::new(self.3);
        [s0, s1, s2, s3]
    }
}
