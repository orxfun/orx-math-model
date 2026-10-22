use crate::data::par_data::par_and_data::ParDataCore;
use alloc::boxed::Box;

pub trait ParDataCollection<'m> {
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn ParDataCore<'m> + 'm>>;
}

impl<'m, P0> ParDataCollection<'m> for P0
where
    P0: ParDataCore<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn ParDataCore<'m> + 'm>> {
        let p0: Box<dyn ParDataCore<'m> + 'm> = Box::new(self);
        core::iter::once(p0)
    }
}

impl<'m, P0, P1> ParDataCollection<'m> for (P0, P1)
where
    P0: ParDataCore<'m> + 'm,
    P1: ParDataCore<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn ParDataCore<'m> + 'm>> {
        let p0: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.0);
        let p1: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.1);
        [p0, p1]
    }
}

impl<'m, P0, P1, P2> ParDataCollection<'m> for (P0, P1, P2)
where
    P0: ParDataCore<'m> + 'm,
    P1: ParDataCore<'m> + 'm,
    P2: ParDataCore<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn ParDataCore<'m> + 'm>> {
        let p0: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.0);
        let p1: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.1);
        let p2: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.2);
        [p0, p1, p2]
    }
}

impl<'m, P0, P1, P2, P3> ParDataCollection<'m> for (P0, P1, P2, P3)
where
    P0: ParDataCore<'m> + 'm,
    P1: ParDataCore<'m> + 'm,
    P2: ParDataCore<'m> + 'm,
    P3: ParDataCore<'m> + 'm,
{
    fn into_iter(self) -> impl IntoIterator<Item = Box<dyn ParDataCore<'m> + 'm>> {
        let p0: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.0);
        let p1: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.1);
        let p2: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.2);
        let p3: Box<dyn ParDataCore<'m> + 'm> = Box::new(self.3);
        [p0, p1, p2, p3]
    }
}
