use core::{iter::Copied, slice::Iter};

// TODO: safety notes, T must be copy and no-drop

#[derive(derive_new::new)]
pub struct ElementsIterCore<T, I>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    elements_iter: I,
    slot: *mut T,
}

impl<T, I> Iterator for ElementsIterCore<T, I>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.elements_iter.next().map(|x| {
            unsafe { self.slot.write(x) };
        })
    }
}

pub type ElementsIter<'a, T> = ElementsIterCore<T, Copied<Iter<'a, T>>>;
