use crate::{new_bag, sets::SetTyp, Bag, Elements, Res, Slot};
use crate::{MetaSet, Model, Ptr};
use alloc::string::String;
use core::mem::{transmute, MaybeUninit};
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub enum IdxSlot<'d> {
    CatIdxInt(Slot<'d, usize>),
    CatIdxStr(Slot<'d, &'d String>),
    OrdIdx(Slot<'d, i64>),
}

#[derive(Debug, PartialEq)]
pub enum IdxVal<'d> {
    CatInt(usize),
    CatStr(&'d String),
    Ord(i64),
}

impl<'d> IdxSlot<'d> {
    #[cfg(test)]
    pub fn test_value(&self) -> IdxVal<'d> {
        match self {
            Self::CatIdxInt(x) => IdxVal::CatInt(*x.get()),
            Self::CatIdxStr(x) => IdxVal::CatStr(*x.get()),
            Self::OrdIdx(x) => IdxVal::Ord(*x.get()),
        }
    }

    pub fn val_ptr<I: MetaSet>(&self) -> Ptr<'d, I::IdxRaw<'d>> {
        match self {
            IdxSlot::CatIdxInt(s) => unsafe { transmute(s.ptr()) },
            IdxSlot::CatIdxStr(s) => unsafe { transmute(s.ptr()) },
            IdxSlot::OrdIdx(s) => unsafe { transmute(s.ptr()) },
        }
    }
}

// collection

pub struct IdxSlots<'d> {
    cat_idx_int: Bag<usize>,
    cat_idx_str: Bag<&'d String>,
    ord_idx: Bag<i64>,
}

impl Default for IdxSlots<'_> {
    fn default() -> Self {
        Self {
            cat_idx_int: new_bag(),
            cat_idx_str: new_bag(),
            ord_idx: new_bag(),
        }
    }
}

impl<'d> IdxSlots<'d> {
    pub fn push(&self, m: &Model<'d>, t: SetTyp) -> IdxSlot<'d> {
        match t {
            SetTyp::CatInt => {
                let el = self.cat_idx_int.push(0);
                IdxSlot::CatIdxInt(Slot::new(m, el))
            }
            SetTyp::CatStr => {
                let el = self.cat_idx_str.push(unsafe { empty_string() });
                IdxSlot::CatIdxStr(Slot::new(m, el))
            }
            SetTyp::Ord => {
                let el = self.ord_idx.push(0);
                IdxSlot::OrdIdx(Slot::new(m, el))
            }
        }
    }
}

pub unsafe fn empty_string() -> &'static String {
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    static mut STRING: MaybeUninit<String> = MaybeUninit::uninit();

    if !INITIALIZED.load(Ordering::Acquire) {
        unsafe { STRING.write(Default::default()) };
        INITIALIZED.store(true, Ordering::Release);
    }
    unsafe { STRING.assume_init_ref() }
}
