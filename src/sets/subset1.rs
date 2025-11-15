use crate::sets::element::Element;
use crate::sets::Set1;

pub struct Subset1<'m, T1, F>
where
    T1: Element,
    F: Fn(&T1) -> bool,
{
    set: Set1<'m, T1>,
    filter: F,
}
