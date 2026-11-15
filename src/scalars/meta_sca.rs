use crate::scalars::ValTyp;

// enums

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScaTyp {
    Int,
    Num,
    Var,
}

impl From<ValTyp> for ScaTyp {
    fn from(value: ValTyp) -> Self {
        match value {
            ValTyp::Int => Self::Int,
            ValTyp::Num => Self::Num,
        }
    }
}
