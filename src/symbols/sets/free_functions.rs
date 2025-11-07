use crate::symbols::sets::IndependentSetCollection;

pub fn set_of<'m, S>(sets: S) -> S::Set
where
    S: IndependentSetCollection<'m>,
{
    sets.model().dep_set(sets)
}
