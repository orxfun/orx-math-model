use super::set_elements::{Elements, SetElements};
use crate::data_structs::current_set_indices::CurrentSetElements;

#[derive(Clone, Copy, Debug)]
pub(crate) enum BoundFrom {
    Above,
    Below,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum BoundEnd {
    Inclusive,
    Exclusive,
}

#[derive(Clone)]
pub struct BoundedSet {
    pub(crate) bound_from: BoundFrom,
    pub(crate) bound_type: BoundEnd,
}

impl BoundedSet {
    pub(crate) fn elements<'a>(
        &self,
        unbounded_set: &'a SetElements,
        dependent_set_element: usize,
        current: &CurrentSetElements,
    ) -> Elements<'a> {
        use BoundEnd::*;
        use BoundFrom::*;

        let unbounded_iter = unbounded_set.elements_for(current);

        match self.bound_from {
            Above => match self.bound_type {
                Inclusive => Box::new(unbounded_iter.filter(move |x| x <= &dependent_set_element)),
                Exclusive => Box::new(unbounded_iter.filter(move |x| x < &dependent_set_element)),
            },
            Below => match self.bound_type {
                Inclusive => Box::new(unbounded_iter.filter(move |x| x >= &dependent_set_element)),
                Exclusive => Box::new(unbounded_iter.filter(move |x| x > &dependent_set_element)),
            },
        }
    }

    pub(super) fn get_relation_str(&self, bounding_set_key: &str) -> String {
        use BoundEnd::*;
        use BoundFrom::*;

        let rel = match self.bound_from {
            Above => "<",
            Below => ">",
        };

        let eq = match self.bound_type {
            Inclusive => "=",
            Exclusive => "",
        };

        format!("{}{} {}", rel, eq, bounding_set_key)
    }
}
