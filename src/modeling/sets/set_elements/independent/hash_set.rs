use crate::impl_elements_from_iter_copied;
use std::collections::HashSet;

impl_elements_from_iter_copied!([], HashSet<usize>, []);

#[cfg(test)]
mod tests {
    use crate::modeling::sets::set_elements::elements::Elements;
    use std::collections::HashSet;

    fn take<E: Elements>(e: E) {
        assert_eq!(e.elements(&Default::default()).count(), 4);
        assert_eq!(e.elements(&Default::default()).sum::<usize>(), 10);
    }

    #[test]
    fn hash_set_as_elements() {
        let mut set: HashSet<usize> = [2, 4, 1, 3].into_iter().collect();
        take(&mut set);
        take(&set);
        take(set);
    }
}
