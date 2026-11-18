use crate::scalars::val::UFunVal;
use crate::scalars::{FVal, Val, ValTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub struct DVal<'d> {
    typ: ValTyp,
    data: UFunVal<'d>,
}

impl<'d> Drop for DVal<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.typ {
                ValTyp::Int => ManuallyDrop::drop(&mut self.data.int),
                ValTyp::Num => ManuallyDrop::drop(&mut self.data.num),
            }
        }
    }
}

impl<'d> DVal<'d> {
    pub fn new<V: Val>(fun: FVal<'d, V>) -> Self {
        let typ = V::val_typ();
        let data = V::into_fun_union(fun);
        Self { typ, data }
    }

    pub fn typ(&self) -> ValTyp {
        self.typ
    }

    pub fn add<'m, V: Val>(&self, rhs: V::Raw) -> Self {
        match (self.typ, V::val_typ()) {
            (ValTyp::Int, ValTyp::Int) => {
                let f1: &Box<dyn Fn() -> f64> = unsafe { &self.data.int };
                let fun = move || {
                    // let x2: i64 = unsafe { transmute(rhs) };
                    f1()
                };

                // let fun: Box<dyn Fn() -> i64> =
                // Box::new(move || f1() + unsafe { transmute::<V::Raw, i64>(rhs()) });
                // let fun: &ManuallyDrop<Box<dyn Fn() -> i64>>
                //
                todo!()
            }
            _ => todo!(),
        }
    }

    #[inline(always)]
    pub fn eval<V: Val>(&self) -> V::Raw {
        debug_assert_eq!(V::val_typ(), self.typ);
        let fun = V::fun_from_union(&self.data);
        fun()
    }
}
