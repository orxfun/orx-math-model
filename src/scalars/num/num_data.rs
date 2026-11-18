use crate::scalars::num::meta_val::round_f64;
use crate::scalars::num::{FInt, FNum};
use crate::{AsF64, AsI64, Ptr, ValTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

#[derive(derive_new::new)]
pub struct DNum<'d> {
    pub typ: ValTyp,
    fun: FNum<'d>,
}

impl<'d> DNum<'d> {
    pub fn new_num<V: AsF64>(fun: impl Fn() -> V + 'd) -> Self {
        let fun = Box::new(move || fun().as_f64());
        let typ = ValTyp::Num;
        Self { typ, fun }
    }

    pub fn new_int<V: AsI64>(fun: impl Fn() -> V + 'd) -> Self {
        let fun = Box::new(move || fun().as_f64());
        let typ = ValTyp::Int;
        Self { typ, fun }
    }

    #[inline(always)]
    pub fn eval(&self) -> f64 {
        (self.fun)()
    }

    #[inline(always)]
    pub fn eval_int(&self) -> i64 {
        round_f64((self.fun)())
    }

    // ops

    fn ops_typ(lhs: Ptr<'d, Self>, rhs: Ptr<'d, Self>) -> ValTyp {
        match (lhs.get().typ, rhs.get().typ) {
            (ValTyp::Int, ValTyp::Int) => ValTyp::Int,
            _ => ValTyp::Num,
        }
    }

    pub fn neg(lhs: Ptr<'d, Self>) -> Self {
        let typ = lhs.get().typ;
        let fun = Box::new(move || -lhs.get().eval());
        Self { typ, fun }
    }

    pub fn recip(lhs: Ptr<'d, Self>) -> Self {
        let typ = lhs.get().typ;
        let fun = Box::new(move || 1.0 / lhs.get().eval());
        Self { typ, fun }
    }

    pub fn add(lhs: Ptr<'d, Self>, rhs: Ptr<'d, Self>) -> Self {
        let typ = Self::ops_typ(lhs, rhs);
        let fun = Box::new(move || lhs.get().eval() + rhs.get().eval());
        Self { typ, fun }
    }

    pub fn sub(lhs: Ptr<'d, Self>, rhs: Ptr<'d, Self>) -> Self {
        let typ = Self::ops_typ(lhs, rhs);
        let fun = Box::new(move || lhs.get().eval() - rhs.get().eval());
        Self { typ, fun }
    }

    pub fn sub_con(typ: ValTyp, lhs: Ptr<'d, Self>, rhs: impl AsF64) -> Self {
        let fun = Box::new(move || lhs.get().eval() - rhs.as_f64());
        Self { typ, fun }
    }

    pub fn mul(lhs: Ptr<'d, Self>, rhs: Ptr<'d, Self>) -> Self {
        let typ = Self::ops_typ(lhs, rhs);
        let fun = Box::new(move || lhs.get().eval() * rhs.get().eval());
        Self { typ, fun }
    }

    pub fn mul_con(lhs: Ptr<'d, Self>, rhs: impl AsF64) -> Self {
        Self {
            typ: ValTyp::Num,
            fun: Box::new(move || lhs.get().eval() * rhs.as_f64()),
        }
    }

    pub fn div(lhs: Ptr<'d, Self>, rhs: Ptr<'d, Self>) -> Self {
        let typ = ValTyp::Num;
        let fun = Box::new(move || lhs.get().eval() / rhs.get().eval());
        Self { typ, fun }
    }
}
