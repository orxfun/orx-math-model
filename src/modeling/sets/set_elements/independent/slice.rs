use crate::impl_elements_from_iter_copied;

impl_elements_from_iter_copied!([], &[usize], []);
impl_elements_from_iter_copied!([], &mut [usize], []);

#[cfg(test)]
mod tests {
    use crate::modeling::sets::set_elements::elements::Elements;
    use alloc::vec;

    fn take<E: Elements>(e: E) {
        assert_eq!(e.elements(&Default::default()).count(), 4);
        assert_eq!(e.elements(&Default::default()).sum::<usize>(), 10);
    }

    #[test]
    fn slice_as_elements() {
        let mut vec = vec![2, 4, 1, 3];

        take(vec.as_slice());
        take(&vec.as_slice());
        take(&mut vec.as_slice());

        take(vec.as_mut_slice());
        take(&vec.as_mut_slice());
        take(&mut vec.as_mut_slice());
    }
}
