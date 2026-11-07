// TODO: safety notes, T must be copy and no-drop

pub struct ElementsIter<T, I>
where
    I: Iterator<Item = T>,
{
    elements_iter: I,
    slot: *mut T,
}

impl<T, I> Iterator for ElementsIter<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.elements_iter.next().map(|x| {
            unsafe { self.slot.write(x) };
        })
    }
}
