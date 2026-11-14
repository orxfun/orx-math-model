use crate::val_idx_pairs::D1ValIdxPair;
use crate::{
    Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val, ValInt, ValNum, ValTyp,
};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;

pub type FP1<'d, V, I0> = Box<dyn Fn(<I0 as Set>::IdxRaw<'d>) -> <V as Val>::Raw + 'd>;

#[repr(C)]
pub union UFunP1V<'d, V: Val> {
    pub cint: ManuallyDrop<FP1<'d, V, SetCatInt>>,
    pub cstr: ManuallyDrop<FP1<'d, V, SetCatStr>>,
    pub ord: ManuallyDrop<FP1<'d, V, SetOrd>>,
}

#[repr(C)]
pub union UFunP1<'d> {
    pub int: ManuallyDrop<UFunP1V<'d, ValInt>>,
    pub num: ManuallyDrop<UFunP1V<'d, ValNum>>,
}

pub struct FunP1<'d> {
    pub val_typ: ValTyp,
    pub i0_typ: SetTyp,
    data: UFunP1<'d>,
}

impl<'d> Drop for FunP1<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.val_typ {
                ValTyp::Int => {
                    let data = &mut self.data.int;
                    match self.i0_typ {
                        SetTyp::CatInt => ManuallyDrop::drop(&mut data.cint),
                        SetTyp::CatStr => ManuallyDrop::drop(&mut data.cstr),
                        SetTyp::Ord => ManuallyDrop::drop(&mut data.ord),
                    }
                }
                ValTyp::Num => {
                    let data = &mut self.data.num;
                    match self.i0_typ {
                        SetTyp::CatInt => ManuallyDrop::drop(&mut data.cint),
                        SetTyp::CatStr => ManuallyDrop::drop(&mut data.cstr),
                        SetTyp::Ord => ManuallyDrop::drop(&mut data.ord),
                    }
                }
            }
        }
    }
}

impl<'d> FunP1<'d> {
    pub fn empty(val_typ: ValTyp, i0_typ: SetTyp) -> Self {
        let data = match (val_typ, i0_typ) {
            (ValTyp::Int, SetTyp::CatInt) => UFunP1 {
                int: ManuallyDrop::new(UFunP1V {
                    cint: ManuallyDrop::new(Box::new(|_| Default::default())),
                }),
            },
            (ValTyp::Int, SetTyp::CatStr) => UFunP1 {
                int: ManuallyDrop::new(UFunP1V {
                    cstr: ManuallyDrop::new(Box::new(|_| Default::default())),
                }),
            },
            (ValTyp::Int, SetTyp::Ord) => UFunP1 {
                int: ManuallyDrop::new(UFunP1V {
                    ord: ManuallyDrop::new(Box::new(|_| Default::default())),
                }),
            },

            (ValTyp::Num, SetTyp::CatInt) => UFunP1 {
                num: ManuallyDrop::new(UFunP1V {
                    cint: ManuallyDrop::new(Box::new(|_| Default::default())),
                }),
            },
            (ValTyp::Num, SetTyp::CatStr) => UFunP1 {
                num: ManuallyDrop::new(UFunP1V {
                    cstr: ManuallyDrop::new(Box::new(|_| Default::default())),
                }),
            },
            (ValTyp::Num, SetTyp::Ord) => UFunP1 {
                num: ManuallyDrop::new(UFunP1V {
                    ord: ManuallyDrop::new(Box::new(|_| Default::default())),
                }),
            },
        };
        Self {
            val_typ,
            i0_typ,
            data,
        }
    }

    pub fn new<V: Val, I0: Set>(fun: FP1<'d, V, I0>) -> Self
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    {
        let (val_typ, i0_typ) = (V::typ(), I0::typ());
        let data = <(V, I0) as D1ValIdxPair>::into_fun_union(fun);
        Self {
            val_typ,
            i0_typ,
            data,
        }
    }

    pub fn get<V: Val, I0: Set>(&self) -> &FP1<'d, V, I0>
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    {
        Res::dbg_assert_same_val(V::typ(), self.val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.i0_typ);
        <(V, I0) as D1ValIdxPair>::fun_from_union(&self.data)
    }
}
