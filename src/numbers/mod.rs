mod val;

pub use val::ValFun;

pub type Int<'m, 'd> = val::Val<'m, 'd, i64>;
pub type Num<'m, 'd> = val::Val<'m, 'd, f64>;
