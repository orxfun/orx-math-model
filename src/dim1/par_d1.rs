use crate::ptr::{RefPtr, ValPtr};
use crate::set_and_idx::{AsIdxIdx, AsSetIdxIdx, AsSetStrIdx, SetAndIdx};
use crate::symbols::{Indices, Int, IntFun, Num, NumFun, SetIdx, SetStr};
use crate::Model;
use crate::{key::Key, val::Val};
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
                let p = unsafe { ValPtr::new(el.val_ptr()) };
                NumFun::new(Box::new(move || fun_d1.val().0(p.val())))
            }
            AsSetIdxIdx::Const(i) => NumFun::new(Box::new(move || fun_d1.val().0(i))),
        };

        self.m.num(f)
    }
}

// S = set-str; V = f64

impl<'m, 'd> ParD1<'m, 'd, SetStr<'m, 'd>, f64> {
    pub(crate) fn at(self, i: impl Into<AsSetStrIdx<'m, 'd>>) -> Num<'m, 'd> {
        let fun_d1 = unsafe { RefPtr::new(self.el.as_ptr()) };

        let f = match i.into() {
            AsSetStrIdx::El(el) => {
                let p = unsafe { ValPtr::new(el.val_ptr()) };
                NumFun::new(Box::new(move || fun_d1.val().0(p.val())))
            }
            AsSetStrIdx::Const(i) => NumFun::new(Box::new(move || fun_d1.val().0(i))),
        };

        self.m.num(f)
    }
}

// S = indices; V = f64

impl<'m, 'd> ParD1<'m, 'd, Indices<'m, 'd>, f64> {
    pub(crate) fn at(self, i: impl Into<AsIdxIdx<'m, 'd>>) -> Num<'m, 'd> {
        let fun_d1 = unsafe { RefPtr::new(self.el.as_ptr()) };

        let f = match i.into() {
            AsIdxIdx::Int(int) => {
                let p = unsafe { RefPtr::new(int.fun_ptr()) };
                NumFun::new(Box::new(move || fun_d1.val().0(p.val().eval())))
            }
            AsIdxIdx::Const(i) => NumFun::new(Box::new(move || fun_d1.val().0(i))),
        };

        self.m.num(f)
    }
}

// S = set-idx; V = i32

impl<'m, 'd> ParD1<'m, 'd, SetIdx<'m, 'd>, i32> {
    pub(crate) fn at(self, i: impl Into<AsSetIdxIdx<'m, 'd>>) -> Int<'m, 'd> {
        let fun_d1 = unsafe { RefPtr::new(self.el.as_ptr()) };

        let f = match i.into() {
            AsSetIdxIdx::El(el) => {
                let p = unsafe { ValPtr::new(el.val_ptr()) };
                IntFun::new(Box::new(move || fun_d1.val().0(p.val())))
            }
            AsSetIdxIdx::Const(i) => IntFun::new(Box::new(move || fun_d1.val().0(i))),
        };

        self.m.int(f)
    }
}

// S = set-str; V = i32

impl<'m, 'd> ParD1<'m, 'd, SetStr<'m, 'd>, i32> {
    pub(crate) fn at(self, i: impl Into<AsSetStrIdx<'m, 'd>>) -> Int<'m, 'd> {
        let fun_d1 = unsafe { RefPtr::new(self.el.as_ptr()) };

        let f = match i.into() {
            AsSetStrIdx::El(el) => {
                let p = unsafe { ValPtr::new(el.val_ptr()) };
                IntFun::new(Box::new(move || fun_d1.val().0(p.val())))
            }
            AsSetStrIdx::Const(i) => IntFun::new(Box::new(move || fun_d1.val().0(i))),
        };

        self.m.int(f)
    }
}

// S = indices; V = i32

impl<'m, 'd> ParD1<'m, 'd, Indices<'m, 'd>, i32> {
    pub(crate) fn at(self, i: impl Into<AsIdxIdx<'m, 'd>>) -> Int<'m, 'd> {
        let fun_d1 = unsafe { RefPtr::new(self.el.as_ptr()) };

        let f = match i.into() {
            AsIdxIdx::Int(int) => {
                let p = unsafe { RefPtr::new(int.fun_ptr()) };
                IntFun::new(Box::new(move || fun_d1.val().0(p.val().eval())))
            }
            AsIdxIdx::Const(i) => IntFun::new(Box::new(move || fun_d1.val().0(i))),
        };

        self.m.int(f)
    }
}
