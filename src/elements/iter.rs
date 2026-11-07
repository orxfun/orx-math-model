use crate::ptr::ValPtr;

// TODO: safety notes, T must be copy and no-drop

#[derive(derive_new::new)]
pub struct ElementsIter<T, I>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    elements_iter: I,
    slot: ValPtr<T>,
}

impl<T, I> Iterator for ElementsIter<T, I>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.elements_iter.next().map(|x| {
            self.slot.write(x);
        })
    }
}
