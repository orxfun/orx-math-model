use crate::{
    Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val, ValInt, ValNum, ValTyp,
};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub type FVal<'d, V> = Box<dyn Fn() -> <V as Val>::Raw + 'd>;

#[repr(C)]
pub union UFunVal<'d> {
    pub int: ManuallyDrop<FVal<'d, ValInt>>,
    pub num: ManuallyDrop<FVal<'d, ValNum>>,
}

pub struct FunVal<'d> {
    pub typ: ValTyp,
    data: UFunVal<'d>,
}

impl<'d> Drop for FunVal<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.typ {
                ValTyp::Int => ManuallyDrop::drop(&mut self.data.int),
                ValTyp::Num => ManuallyDrop::drop(&mut self.data.num),
            }
        }
    }
}

impl<'d> FunVal<'d> {
    pub fn new<V: Val>(fun: FVal<'d, V>) -> Self {
        let fun = ManuallyDrop::new(fun);
        let typ = V::typ();
        let data = match typ {
            ValTyp::Int => UFunVal {
                int: unsafe { transmute(fun) },
            },
            ValTyp::Num => UFunVal {
                num: unsafe { transmute(fun) },
            },
        };
        Self { typ, data }
    }

    pub fn get<V: Val>(&self) -> &FVal<'d, V> {
        Res::dbg_assert_same_val(V::typ(), self.typ);
        V::val_unpack(&self.data)
    }
}
