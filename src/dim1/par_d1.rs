use crate::{key::Key, val::Val, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct ParD1Fun<'d, K, V>(Box<dyn Fn(K) -> V + 'd>)
where
    K: Key,
    V: Val;

impl<'d, K, V> ParD1Fun<'d, K, V>
where
    K: Key,
    V: Val,
{
    pub fn empty() -> Self {
        Self(Box::new(|_| Default::default()))
    }

    pub fn eval(&self, i0: K) -> V {
        self.0(i0)
    }
}

// symbol

#[derive(derive_new::new)]
pub struct ParD1<'m, 'd, K, V>
where
    K: Key,
    V: Val,
{
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParD1Fun<'d, K, V>>,
}
