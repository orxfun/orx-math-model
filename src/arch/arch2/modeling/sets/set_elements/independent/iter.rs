use crate::modeling::{
    current_set_elements::CurrentSetElements, sets::set_elements::elements::Elements,
};

pub struct ElementsIter<I>
where
    I: Iterator<Item = usize> + Clone,
{
    iter: I,
}

impl<I> Elements for ElementsIter<I>
where
    I: Iterator<Item = usize> + Clone,
{
    fn elements(&self, _: &CurrentSetElements) -> impl Iterator<Item = usize> {
        self.iter.clone()
    }
}

pub trait IntoElementsIter {
    fn as_elements(self) -> ElementsIter<impl Iterator<Item = usize> + Clone>;
}

impl<I: Iterator<Item = usize> + Clone> IntoElementsIter for I {
    fn as_elements(self) -> ElementsIter<impl Iterator<Item = usize> + Clone> {
        ElementsIter { iter: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modeling::sets::set_elements::elements::Elements;
    use alloc::vec;

    fn take<E: Elements>(e: E) {
        assert_eq!(e.elements(&Default::default()).count(), 4);
        assert_eq!(e.elements(&Default::default()).sum::<usize>(), 10);
    }

    #[test]
    fn iter_as_elements() {
        let vec = vec![2, 1, 4, 3, 7];

        take(vec.iter().filter(|x| **x < 7).copied().as_elements());
        take(vec.into_iter().filter(|x| x < &7).as_elements());

        let iter = (0..1000)
            .into_iter()
            .filter(|x| x >= &5 && x <= &8)
            .map(|x| x - 4);
        take(iter.as_elements());
    }
}
