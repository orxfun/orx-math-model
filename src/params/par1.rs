use crate::indices::{CatIdx, DIdx, Idx};
use crate::ptr::RefPtr;
use crate::sets::{DSet, Set};
use crate::values::{DVal, Num};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

#[derive(Clone, Copy)]
enum State {
    FunIntSet,
    FunStrSet,
    Unset,
}

pub struct DParD1<'d, T> {
    s0: RefPtr<DSet<'d>>,
    fun_int: Box<dyn Fn(usize) -> T + 'd>,
    fun_str: Box<dyn Fn(&'d str) -> T + 'd>,
    state: State,
}

impl<'d, T> DParD1<'d, T> {
    pub fn empty<'m>(set: impl Into<Set<'m, 'd>>) -> Self
    where
        T: Default,
        'd: 'm,
    {
        Self {
            s0: RefPtr::from(set.into().el()),
            fun_int: Box::new(|_| T::default()),
            fun_str: Box::new(|_| T::default()),
            state: State::Unset,
        }
    }

    fn fun_int_ptr(&self) -> RefPtr<dyn Fn(usize) -> T + 'd> {
        unsafe { RefPtr::new(self.fun_int.as_ref() as *const dyn Fn(usize) -> T) }
    }

    fn fun_str_ptr(&self) -> RefPtr<dyn Fn(&'d str) -> T + 'd> {
        unsafe { RefPtr::new(self.fun_str.as_ref() as *const dyn Fn(&'d str) -> T) }
    }
}

// sym

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct ParD1<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DParD1<'d, T>>,
}

impl<'m, 'd> ParD1<'m, 'd, f64> {
    pub fn at(self, i0: impl Into<Idx<'m, 'd>>) -> Num<'m, 'd> {
        // TODO: check if i0 and s0 agree
        let i0 = i0.into();
        let dpar = unsafe { self.el.get() };
        let m = self.m;

        match i0 {
            Idx::CatInt(i0) => {
                let i0 = unsafe { i0.el.get() }.val_ptr_int();
                let fun = dpar.fun_int_ptr();
                m.fun_num(DVal::new(Box::new(move || fun.val()(i0.val()))))
            }
            Idx::CatStr(i0) => {
                let i0 = unsafe { i0.el.get() }.val_ptr_str();
                let fun = dpar.fun_str_ptr();
                m.fun_num(DVal::new(Box::new(move || fun.val()(i0.val()))))
            }
            Idx::Ord(i0) => {
                let i0 = RefPtr::from(i0.el);
                let fun = dpar.fun_int_ptr();
                m.fun_num(DVal::new(Box::new(move || {
                    fun.val()(i0.val().eval() as usize)
                })))
            }
        }
    }
}
