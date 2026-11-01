use crate::{ptr::RefPtr, sets::Set};
use alloc::boxed::Box;

// fun

#[derive(derive_new::new)]
pub struct ParD1Fun<'d, S0, T>
where
    S0: Set,
    T: Default,
{
    s0: RefPtr<S0::Fun>,
    fun: Box<dyn Fn(S0::RawIdx) -> T + 'd>,
}

impl<'d, S0, T> ParD1Fun<'d, S0, T>
where
    S0: Set,
    T: Default,
{
    pub fn empty(set: S0) -> Self {
        let s0 = set.fun_ptr();
        let fun = Box::new(|_: S0::RawIdx| Default::default());
        Self { s0, fun }
    }
}

// enum Accessor<'d, T> {
//     Int(Box<dyn Fn(usize) -> T + 'd>),
//     Str(Box<dyn Fn(&'d str) -> T + 'd>),
// }

// pub struct ParD1Fun<'d, T> {
//     s0: SetFunPtr<'d>,
//     fun: Accessor<'d, T>,
// }

// impl<'d, T> ParD1Fun<'d, T> {
//     pub fn empty<'m>(set: Set<'m, 'd>) -> Self {
//         tod
//     }
// }
