#[derive(derive_new::new, Clone, Copy, PartialEq)]
pub struct Bounds<T> {
    lb: Option<T>,
    ub: Option<T>,
}

impl Default for Bounds<i64> {
    fn default() -> Self {
        Self {
            lb: Some(0),
            ub: None,
        }
    }
}

impl Default for Bounds<f64> {
    fn default() -> Self {
        Self {
            lb: Some(0.0),
            ub: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum VarType {
    Cont(Bounds<f64>),
    Binary,
    Int(Bounds<i64>),
}
