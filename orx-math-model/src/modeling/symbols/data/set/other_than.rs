use super::set_elements::{Elements, SetElements};
use crate::data_structs::current_set_indices::CurrentSetElements;

pub(crate) fn elements_other_than<'a>(
    unbounded_set: &'a SetElements,
    dependent_set_element: usize,
    current: &CurrentSetElements,
) -> Elements<'a> {
    let unbounded_iter = unbounded_set.elements_for(current);
    Box::new(unbounded_iter.filter(move |x| *x != dependent_set_element))
}
