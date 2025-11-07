use crate::symbols::sets::SetCore;
use core::marker::PhantomData;

// trait

pub trait StQueue<'m> {
    type PushBack<Elem>: StQueue<'m>
    where
        Elem: Into<SetCore<'m>>;

    type Front: Into<SetCore<'m>>;

    type Back: StQueue<'m>;

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
