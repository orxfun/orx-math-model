use crate::{binding::Binding, elements::Elements, index_types::IdxIn};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum DSet<'d> {
    CatUsize(Binding, Box<dyn Fn(&mut Elements<usize>) + 'd>),
    CatStr(Binding, Box<dyn Fn(&mut Elements<&'d str>) + 'd>),
    Ord(Binding, Box<dyn Fn(&mut Elements<i64>) + 'd>),
}

impl<'d> DSet<'d> {
    pub fn new_cat() -> Self {
        Self::CatUsize(Binding::Empty, Box::new(|_| {}))
    }

    pub fn new_ord() -> Self {
        Self::Ord(Binding::Empty, Box::new(|_| {}))
    }

    pub fn is_bound(&self) -> bool {
        *(match self {
            DSet::CatUsize(b, _) => b,
            DSet::CatStr(b, _) => b,
            DSet::Ord(b, _) => b,
        }) == Binding::Bound
    }

    pub fn bind<I: IdxIn<'d>>(&mut self, fun: impl Fn(&mut Elements<I>) + 'd) {
        I::set_bind(self, fun);
    }

    pub fn bind_from<I, J>(&mut self, fun: impl Fn() -> J + 'd)
    where
        I: IdxIn<'d>,
        J: IntoIterator<Item = I>,
    {
        let fun = move |e: &mut Elements<I>| e.fill(fun());
        I::set_bind(self, fun);
    }

    pub fn fill<I: IdxIn<'d>>(&self, elements: &mut Elements<I>) {
        I::set_fill(self, elements);
    }
}
