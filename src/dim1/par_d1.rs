use crate::{key::Key, val::Val};
use alloc::boxed::Box;

// fun

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
