use crate::symbols::Int;

// int

pub enum PseudoInt<'m, 'd> {
    Raw(usize),
    Int(Int<'m, 'd>),
}

impl<'m, 'd> From<usize> for PseudoInt<'m, 'd> {
    fn from(value: usize) -> Self {
        Self::Raw(value)
    }
}

impl<'m, 'd, I> From<I> for PseudoInt<'m, 'd>
where
    I: Into<Int<'m, 'd>>,
{
    fn from(value: I) -> Self {
        Self::Int(value.into())
    }
}
