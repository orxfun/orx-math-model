use crate::ptr::RefPtr;
use crate::values::{DVal, Num, Value};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum FunParD1<'d, T> {
    Int(Box<dyn Fn(usize) -> T + 'd>),
    Str(Box<dyn Fn(&'d str) -> T + 'd>),
}

impl<'d, T> From<Box<dyn Fn(usize) -> T + 'd>> for FunParD1<'d, T> {
    fn from(value: Box<dyn Fn(usize) -> T + 'd>) -> Self {
        Self::Int(value)
    }
}

impl<'d, T> From<Box<dyn Fn(&'d str) -> T + 'd>> for FunParD1<'d, T> {
    fn from(value: Box<dyn Fn(&'d str) -> T + 'd>) -> Self {
        Self::Str(value)
    }
}

pub enum InputVariant {
    Int,
    Str,
}

pub trait In<'d>: Sized {
    fn variant() -> InputVariant;

    fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd);
}

impl<'d> In<'d> for usize {
    fn variant() -> InputVariant {
        InputVariant::Int
    }

    fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd) {
        data.fun_int = Box::new(fun);
    }
}

impl<'d> In<'d> for &'d str {
    fn variant() -> InputVariant {
        InputVariant::Str
    }

    fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd) {
        data.fun_str = Box::new(fun);
    }
}

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

    pub fn bind(&mut self, fun: impl Into<FunParD1<'d, T>>) {
        match fun.into() {
            FunParD1::Int(f) => self.fun_int = f,
            FunParD1::Str(f) => self.fun_str = f,
        }
    }

    pub fn bind2<I>(&mut self, fun: impl Fn(I) -> T + 'd)
    where
        I: In<'d>,
    {
        I::update_dpar_d1(self, fun);
    }
}

// sym

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct ParD1<'m, 'd, T: Value> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DParD1<'d, T>>,
}

impl<'m, 'd, T: Value> ParD1<'m, 'd, T> {
    pub fn at(self, i0: impl Into<Idx<'m, 'd>>) -> T::Sym<'m, 'd> {
        // TODO: check if i0 and s0 agree
        let i0 = i0.into();
        let dpar = unsafe { self.el.get() };
        let m = self.m;

        match i0 {
            Idx::CatInt(i0) => {
                let i0 = unsafe { i0.el.get() }.val_ptr_int();
                let fun = dpar.fun_int_ptr();
                let data = DVal::new(Box::new(move || fun.val()(i0.val())));
                Value::add_fun_to_model(m, data)
            }
            Idx::CatStr(i0) => {
                let i0 = unsafe { i0.el.get() }.val_ptr_str();
                let fun = dpar.fun_str_ptr();
                let data = DVal::new(Box::new(move || fun.val()(i0.val())));
                Value::add_fun_to_model(m, data)
            }
            Idx::Ord(i0) => {
                let i0 = RefPtr::from(i0.el);
                let fun = dpar.fun_int_ptr();
                let data = DVal::new(Box::new(move || fun.val()(i0.val().eval() as usize)));
                Value::add_fun_to_model(m, data)
            }
        }
    }
}
