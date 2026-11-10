// use crate::values::{DVal, Int, Num};
// use crate::Model;
// use alloc::boxed::Box;

// pub trait Value: Copy + Default + 'static {
//     type Eval<'d>;

//     type Sym<'m, 'd>
//     where
//         'd: 'm;

//     fn add_fun_to_model<'d, 'm>(m: &'m Model<'d>, data: DVal<'d, Self>) -> Self::Sym<'m, 'd>;
// }

// impl Value for i64 {
//     type Eval<'d> = Box<dyn Fn() -> Self + 'd>;

//     type Sym<'m, 'd>
//         = Int<'m, 'd>
//     where
//         'd: 'm;

//     fn add_fun_to_model<'d, 'm>(m: &'m Model<'d>, data: DVal<'d, Self>) -> Self::Sym<'m, 'd> {
//         m.fun_int(data)
//     }
// }

// impl Value for f64 {
//     type Eval<'d> = Box<dyn Fn() -> Self + 'd>;

//     type Sym<'m, 'd>
//         = Num<'m, 'd>
//     where
//         'd: 'm;

//     fn add_fun_to_model<'d, 'm>(m: &'m Model<'d>, data: DVal<'d, Self>) -> Self::Sym<'m, 'd> {
//         m.fun_num(data)
//     }
// }

// // enums

// pub enum DValKinds<'d> {
//     Int(DVal<'d, i64>),
//     Num(DVal<'d, f64>),
// }

// impl<'d> From<DVal<'d, i64>> for DValKinds<'d> {
//     fn from(value: DVal<'d, i64>) -> Self {
//         Self::Int(value)
//     }
// }

// impl<'d> From<DVal<'d, f64>> for DValKinds<'d> {
//     fn from(value: DVal<'d, f64>) -> Self {
//         Self::Num(value)
//     }
// }

// impl<'d> DValKinds<'d> {
//     pub fn into_int(self) -> DVal<'d, i64> {
//         match self {
//             DValKinds::Int(x) => x,
//             _ => unreachable!(),
//         }
//     }

//     pub fn into_num(self) -> DVal<'d, f64> {
//         match self {
//             DValKinds::Num(x) => x,
//             _ => unreachable!(),
//         }
//     }
// }
