use crate::{
    key::Key,
    ptr::RefPtr,
    set_and_idx::{AsSetIdxIdx, SetAndIdx},
    symbols::{Num, NumFun, SetIdx},
    val::Val,
    Model,
};
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
pub struct ParD1<'m, 'd, S, V>
where
    S: SetAndIdx,
    V: Val,
{
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParD1Fun<'d, S::Key, V>>,
}

impl<'m, 'd, S, V> ParD1<'m, 'd, S, V>
where
    S: SetAndIdx,
    V: Val,
{
    fn fun_ptr(&self) -> RefPtr<ParD1Fun<'d, S::Key, V>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }

    // pub(crate) fn at(self, i: impl Into<S::AsIdx>) -> V::Sca<'m, 'd> {
    //     todo!()
    // }
}

// S = set-idx; V = f64

impl<'m, 'd> ParD1<'m, 'd, SetIdx<'m, 'd>, f64> {
    pub(crate) fn at(self, i: impl Into<AsSetIdxIdx<'m, 'd>>) -> Num<'m, 'd> {
        let fun_d1 = unsafe { RefPtr::new(self.el.as_ptr()) };

        let f = match i.into() {
            AsSetIdxIdx::El(el) => {
                //
                todo!()
            }
            AsSetIdxIdx::Const(i) => NumFun::new(Box::new(move || fun_d1.val().0(i))),
        };

        self.m.num(f)
    }
}
