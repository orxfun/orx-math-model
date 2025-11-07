use crate::{
    symbols::{pars::ParData, sets::SetCore, DependentSetIndices, Symbol},
    Model, Par, Set,
};
use core::marker::PhantomData;

// trait

pub trait StQueue<'m> {
    type PushBack<Elem>: StQueue<'m>
    where
        Elem: Into<SetCore<'m>>;

    type Front: Into<SetCore<'m>>;

    type Back: StQueue<'m>;

    const LEN: usize;

    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>
    where
        Elem: Into<SetCore<'m>>;
}

// single

pub struct QueueSingle<'m, F>
where
    F: Into<SetCore<'m>>,
{
    f: F,
    p: PhantomData<&'m ()>,
}

impl<'m, F> StQueue<'m> for QueueSingle<'m, F>
where
    F: Into<SetCore<'m>>,
{
    type PushBack<Elem>
        = QueueMulti<'m, F, QueueSingle<'m, Elem>>
    where
        Elem: Into<SetCore<'m>>;

    type Front = F;

    type Back = Self;
    const LEN: usize = 1;

    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>
    where
        Elem: Into<SetCore<'m>>,
    {
        QueueMulti {
            f: self.f,
            b: QueueSingle {
                f: element,
                p: PhantomData,
            },
            p: PhantomData,
        }
    }
}

// multi

pub struct QueueMulti<'m, F, B>
where
    F: Into<SetCore<'m>>,
    B: StQueue<'m>,
{
    f: F,
    b: B,
    p: PhantomData<&'m ()>,
}

impl<'m, F, B> StQueue<'m> for QueueMulti<'m, F, B>
where
    F: Into<SetCore<'m>>,
    B: StQueue<'m>,
{
    type PushBack<Elem>
        = QueueMulti<'m, F, B::PushBack<Elem>>
    where
        Elem: Into<SetCore<'m>>;

    type Front = F;

    type Back = B;
    const LEN: usize = 1 + B::LEN;

    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>
    where
        Elem: Into<SetCore<'m>>,
    {
        QueueMulti {
            f: self.f,
            b: self.b.push(element),
            p: PhantomData,
        }
    }
}

// tuple

impl<'m, X1> From<X1> for QueueSingle<'m, X1>
where
    X1: Into<SetCore<'m>>,
{
    fn from(value: X1) -> Self {
        QueueSingle {
            f: value,
            p: PhantomData,
        }
    }
}

impl<'m, X1, X2> From<(X1, X2)> for QueueMulti<'m, X1, QueueSingle<'m, X2>>
where
    X1: Into<SetCore<'m>>,
    X2: Into<SetCore<'m>>,
{
    fn from((x1, x2): (X1, X2)) -> Self {
        QueueSingle::from(x1).push(x2)
    }
}

impl Model {
    pub(crate) fn par2<'m, const N: usize, Q, T>(&'m self, sets: T) -> Par<'m, N>
    where
        Q: StQueue<'m>,
        T: Into<Q>,
    {
        // let dep = DependentSetIndices::new(sets.into_iter());
        // let data = ParData::new(dep);
        // self.data.pars.push(self, Symbol::new(data)).with_dim()
        todo!()
    }
}

// tuple 2

impl<'m, const N: usize> From<Set<'m, N>> for [SetCore<'m>; 1] {
    fn from(x1: Set<'m, N>) -> Self {
        [x1.into()]
    }
}

#[test]
fn abc() {
    use super::*;
    use crate::*;

    let m = Model::new();
    let i = m.set();
    let j = set_of([i]);

    let sets = QueueSingle {
        f: i,
        p: PhantomData,
    };
    type IJ<'m> = QueueMulti<'m, Set<'m>, QueueSingle<'m, Set<'m, 1>>>;

    let sets: IJ = sets.push(j);

    let sets: IJ = (i, j).into();
    // m.par2::<2, QueueMulti<'_, Set<'_>, QueueMulti<'_, Set<'_>, QueueSingle<'_, Set<'_, 1>>>>, (Set<'_>, Set<'_, 1>)>((i, j));

    m.par2::<2, IJ, _>((i, j));
}
