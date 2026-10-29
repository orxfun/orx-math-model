use crate::{val::Val, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct ParD1Fun<'d, V>(Box<dyn Fn(usize) -> V + 'd>)
where
    V: Val;

impl<'d, V> ParD1Fun<'d, V>
where
    V: Val,
{
    pub fn empty() -> Self {
        Self(Box::new(|_| Default::default()))
    }

    pub fn eval(&self, i0: usize) -> V {
        self.0(i0)
    }
}

// symbol

#[derive(derive_new::new)]
pub struct ParD1<'m, 'd, V>
where
    V: Val,
{
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParD1Fun<'d, V>>,
}
