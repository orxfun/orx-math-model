use crate::values::ValVariant;

pub trait Value: Into<ValVariant> {}

// int

impl Value for i64 {}

impl From<i64> for ValVariant {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

// num

impl Value for f64 {}

impl From<f64> for ValVariant {
    fn from(value: f64) -> Self {
        Self::Num(value)
    }
}
