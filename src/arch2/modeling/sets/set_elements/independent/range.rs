use crate::modeling::{
    current_set_elements::CurrentSetElements, sets::set_elements::elements::Elements,
};
use core::ops::Range;

impl Elements for Range<usize> {
    fn elements(&self, _: &CurrentSetElements) -> impl Iterator<Item = usize> {
        self.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::modeling::sets::set_elements::elements::Elements;

    fn take<E: Elements>(e: E) {
        assert_eq!(e.elements(&Default::default()).count(), 4);
        assert_eq!(e.elements(&Default::default()).sum::<usize>(), 10);
    }

    #[test]
    fn range_as_elements() {
        take(1..5);
    }
}
