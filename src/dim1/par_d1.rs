use crate::{set_and_idx::SetAndIdx, val::Val, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct ParD1Fun<'d, S, V>(Box<dyn Fn(S::Key) -> V + 'd>)
where
    S: SetAndIdx,
    V: Val;

impl<'d, S, V> ParD1Fun<'d, S, V>
where
    S: SetAndIdx,
    V: Val,
{
    pub fn empty() -> Self {
        Self(Box::new(|_| Default::default()))
    }

    pub fn eval(&self, i0: S::Key) -> V {
        self.0(i0)
    }
}

// symbol

#[derive(derive_new::new)]
pub struct ParD1<'m, 'd, S, V>
where
    S: SetAndIdx,
    V: Val,
{
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParD1Fun<'d, S, V>>,
}

impl<'m, 'd, S, V> ParD1<'m, 'd, S, V>
where
    S: SetAndIdx,
    V: Val,
{
    pub(crate) fn at(self, i: impl Into<S::AsIdx>) -> V::Sca<'m, 'd> {
        todo!()
    }
}
