use crate::impl_elements_from_iter_copied;

impl_elements_from_iter_copied!(N, [], [usize; N], []);

#[cfg(test)]
mod tests {
    use crate::modeling::sets::set_elements::elements::Elements;

    fn take<E: Elements>(e: E) {
        assert_eq!(e.elements(&Default::default()).count(), 4);
        assert_eq!(e.elements(&Default::default()).sum::<usize>(), 10);
    }

    #[test]
    fn array_as_elements() {
        let mut array = [2, 4, 1, 3];

        take(array);
        take(&array);
        take(&mut array);
    }
}
