mod val;
mod value;

pub use val::DVal;

pub type Num<'m, 'd> = val::Val<'m, 'd, f64>;
pub type Int<'m, 'd> = val::Val<'m, 'd, i64>;
