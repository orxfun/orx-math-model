#[derive(derive_new::new, Clone, Copy, PartialEq)]
pub struct Bounds<T> {
    lb: T,
    ub: T,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VarType {
    Continuous(Bounds<f64>),
    Binary,
    Int(Bounds<i64>),
}
