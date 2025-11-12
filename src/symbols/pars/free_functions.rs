use crate::symbols::sets::SetCollection;

pub fn par<'m, S>(sets: S) -> S::Par
where
    S: SetCollection<'m>,
{
    sets.model().par_of(sets)
}
