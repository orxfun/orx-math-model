#[derive(derive_new::new, Clone, Copy, PartialEq)]
pub struct BoundsOld<T> {
    lb: Option<T>,
    ub: Option<T>,
}

impl Default for BoundsOld<i64> {
    fn default() -> Self {
        Self {
            lb: Some(0),
            ub: None,
        }
    }
}

impl Default for BoundsOld<f64> {
    fn default() -> Self {
        Self {
            lb: Some(0.0),
            ub: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum VarType {
    Cont(BoundsOld<f64>),
    Binary,
    Int(BoundsOld<i64>),
}
