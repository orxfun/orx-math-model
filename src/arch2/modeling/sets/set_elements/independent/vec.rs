use crate::impl_elements_from_iter_copied;
use alloc::vec::Vec;

impl_elements_from_iter_copied!([], Vec<usize>, []);

#[cfg(test)]
mod tests {
    use crate::modeling::sets::set_elements::elements::Elements;
    use alloc::vec;

    fn take<E: Elements>(e: E) {
        assert_eq!(e.elements(&Default::default()).count(), 4);
        assert_eq!(e.elements(&Default::default()).sum::<usize>(), 10);
    }

    #[test]
    fn vec_as_elements() {
        let mut vec = vec![2, 4, 1, 3];
        take(&mut vec);
        take(&vec);
        take(vec);
    }
}
