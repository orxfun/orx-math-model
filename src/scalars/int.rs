use crate::{
    model::Model,
    pars::{IntPar0, IntPar0Data},
    symbol::Symbol,
};
use alloc::boxed::Box;

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'a>(pub(crate) Symbol<'a, IntData>);

#[derive(derive_new::new)]
pub struct IntData {
    value: Box<dyn Fn() -> i64>,
}

// from

impl<'a> From<IntPar0<'a>> for Int<'a> {
    fn from(value: IntPar0<'a>) -> Self {
        let m: &'a Model = value.0.m;
        let d: &'a IntPar0Data = value.0.d;
        let value = Box::new(move || d.value);
        m.int(value);
        todo!()
    }
}

// impl<'a> From<IntPar0<'a>> for Int<'a> {
//     fn from(value: IntPar0<'a>) -> Self {
//         let (m, pos) = (value.m, value.pos);
//         let create = Box::new(move |m: &Model| m.pars.int_par0[pos].value);
//         let def = value.d.def.clone();
//         m.scalars.add_int(m, def, create)
//     }
// }
