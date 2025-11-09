use crate::SetAndData;
use alloc::boxed::Box;

pub trait SetDataCollection<'m> {
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetAndData<'m> + 'm>>;
}

impl<'m, S1> SetDataCollection<'m> for S1
where
    S1: SetAndData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetAndData<'m> + 'm>> {
        let s0: Box<dyn SetAndData<'m> + 'm> = Box::new(self);
        core::iter::once(s0)
    }
}

impl<'m, S1, S2> SetDataCollection<'m> for (S1, S2)
where
    S1: SetAndData<'m> + 'm,
    S2: SetAndData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetAndData<'m> + 'm>> {
        let s0: Box<dyn SetAndData<'m> + 'm> = Box::new(self.0);
        let s1: Box<dyn SetAndData<'m> + 'm> = Box::new(self.1);
        [s0, s1]
    }
}

impl<'m, S1, S2, S3> SetDataCollection<'m> for (S1, S2, S3)
where
    S1: SetAndData<'m> + 'm,
    S2: SetAndData<'m> + 'm,
    S3: SetAndData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetAndData<'m> + 'm>> {
        let s0: Box<dyn SetAndData<'m> + 'm> = Box::new(self.0);
        let s1: Box<dyn SetAndData<'m> + 'm> = Box::new(self.1);
        let s2: Box<dyn SetAndData<'m> + 'm> = Box::new(self.2);
        [s0, s1, s2]
    }
}

impl<'m, S1, S2, S3, S4> SetDataCollection<'m> for (S1, S2, S3, S4)
where
    S1: SetAndData<'m> + 'm,
    S2: SetAndData<'m> + 'm,
    S3: SetAndData<'m> + 'm,
    S4: SetAndData<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn SetAndData<'m> + 'm>> {
        let s0: Box<dyn SetAndData<'m> + 'm> = Box::new(self.0);
        let s1: Box<dyn SetAndData<'m> + 'm> = Box::new(self.1);
        let s2: Box<dyn SetAndData<'m> + 'm> = Box::new(self.2);
        let s3: Box<dyn SetAndData<'m> + 'm> = Box::new(self.3);
        [s0, s1, s2, s3]
    }
}
