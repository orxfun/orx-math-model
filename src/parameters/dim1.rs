use crate::sets::Set;
use alloc::boxed::Box;

// fun

#[derive(derive_new::new)]
pub struct ParD1Fun<'d, S0, T>
where
    S0: Set,
{
    s0: S0::Fun,
    fun: Box<dyn Fn(S0::RawIdx) -> T + 'd>,
}

impl<'d, S0: Set, T> ParD1Fun<'d, S0, T> {
    //
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
//         todo!()
//     }
// }
