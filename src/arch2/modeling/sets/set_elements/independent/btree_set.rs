use crate::impl_elements_from_iter_copied;
use alloc::collections::BTreeSet;

impl_elements_from_iter_copied!([], BTreeSet<usize>, []);

#[cfg(test)]
mod tests {
    use crate::modeling::sets::set_elements::elements::Elements;
    use alloc::collections::BTreeSet;

    fn take<E: Elements>(e: E) {
        assert_eq!(e.elements(&Default::default()).count(), 4);
        assert_eq!(e.elements(&Default::default()).sum::<usize>(), 10);
    }

    #[test]
    fn btree_set_as_elements() {
        let mut set: BTreeSet<usize> = [2, 4, 1, 3].into_iter().collect();
        take(&mut set);
        take(&set);
        take(set);
    }
}
