use crate::{ptr::RefPtr, sets::SetFunPtr, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

pub enum RawParD1Fun<'d, T: Default> {
    Int(Box<dyn Fn(usize) -> T + 'd>),
    Str(Box<dyn Fn(&'d str) -> T + 'd>),
}

pub struct ParD1Fun<'d, T: Default> {
    s0: SetFunPtr<'d>,
    fun: RawParD1Fun<'d, T>,
}

impl<'d, T: Default> ParD1Fun<'d, T> {
    pub fn empty(set: usize) -> Self {
        todo!()
    }
}

// symbol

pub struct ParD1;

// #[derive(derive_new::new)]
// pub struct ParD1Fun<'d, S0, T>
// where
//     S0: Set,
//     T: Default,
// {
//     s0: RefPtr<S0::Fun<'d>>,
//     fun: Box<dyn Fn(S0::RawIdx<'d>) -> T + 'd>,
// }

// impl<'d, S0, T> ParD1Fun<'d, S0, T>
// where
//     S0: Set,
//     T: Default,
// {
//     pub fn empty<'m>(set: S0::Sym<'m, 'd>) -> Self {
//         let s0 = S0::fun_ptr(set);
//         let fun = Box::new(|_: S0::RawIdx<'d>| Default::default());
//         Self { s0, fun }
//     }
// }

// // symbol

// #[derive(derive_new::new, Clone, Copy)]
// #[new(visibility = "pub(crate)")]
// pub struct ParD1<'m, 'd, S0, T>
// where
//     S0: Set,
//     T: Default,
// {
//     pub(crate) m: &'m Model<'d>,
//     pub(crate) el: Elem<'m, ParD1Fun<'d, S0, T>>,
// }
