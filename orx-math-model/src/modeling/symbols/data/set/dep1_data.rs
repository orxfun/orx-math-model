use super::set_elements::Elements;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    rc::Rc,
};

pub(crate) fn dep1_vec_vec(data: &Rc<Vec<Vec<usize>>>, dependent_set_value: usize) -> Elements<'_> {
    Box::new(
        data.get(dependent_set_value)
            .expect("index-out-of-bounds at Vec<Vec<usize>> as dependent Set")
            .iter()
            .cloned(),
    )
}
pub(crate) fn dep1_vec_set(
    data: &Rc<Vec<HashSet<usize>>>,
    dependent_set_value: usize,
) -> Elements<'_> {
    Box::new(
        data.get(dependent_set_value)
            .expect("index-out-of-bounds at Vec<Vec<usize>> as dependent Set")
            .iter()
            .cloned(),
    )
}

pub(crate) fn dep1_hashmap_vec(
    data: &Rc<HashMap<usize, Vec<usize>>>,
    dependent_set_value: usize,
) -> Elements<'_> {
    Box::new(
        data.get(&dependent_set_value)
            .expect("index-out-of-bounds at HashMap<usize, Vec<usize>> as dependent Set")
            .iter()
            .cloned(),
    )
}
pub(crate) fn dep1_hashmap_set(
    data: &Rc<HashMap<usize, HashSet<usize>>>,
    dependent_set_value: usize,
) -> Elements<'_> {
    Box::new(
        data.get(&dependent_set_value)
            .expect("index-out-of-bounds at HashMap<usize, HashSet<usize>> as dependent Set")
            .iter()
            .cloned(),
    )
}

pub(crate) fn dep1_btreemap_vec(
    data: &Rc<BTreeMap<usize, Vec<usize>>>,
    dependent_set_value: usize,
) -> Elements<'_> {
    Box::new(
        data.get(&dependent_set_value)
            .expect("index-out-of-bounds at BTreeMap<usize, Vec<usize>> as dependent Set")
            .iter()
            .cloned(),
    )
}
pub(crate) fn dep1_btreemap_set(
    data: &Rc<BTreeMap<usize, HashSet<usize>>>,
    dependent_set_value: usize,
) -> Elements<'_> {
    Box::new(
        data.get(&dependent_set_value)
            .expect("index-out-of-bounds at BTreeMap<usize, HashSet<usize>> as dependent Set")
            .iter()
            .cloned(),
    )
}
