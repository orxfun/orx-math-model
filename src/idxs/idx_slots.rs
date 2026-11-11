use crate::{new_bag, sets::SetType, Bag, Elements, Res, Slot};
use alloc::string::String;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

pub enum IdxSlot<'d> {
    CatIdxInt(Slot<'d, usize>),
    CatIdxStr(Slot<'d, &'d String>),
    OrdIdx(Slot<'d, i64>),
}

#[derive(Debug, PartialEq)]
pub enum IdxVal<'d> {
    CatIdxInt(usize),
    CatIdxStr(&'d String),
    OrdIdx(i64),
}

impl<'d> IdxSlot<'d> {
    #[cfg(test)]
    pub fn value(&self) -> IdxVal<'d> {
        match self {
            Self::CatIdxInt(x) => IdxVal::CatIdxInt(*x.get()),
            Self::CatIdxStr(x) => IdxVal::CatIdxStr(*x.get()),
            Self::OrdIdx(x) => IdxVal::OrdIdx(*x.get()),
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
    pub fn push(&self, t: SetType) -> IdxSlot<'d> {
        match t {
            SetType::CatSetInt => {
                let el = self.cat_idx_int.push(0);
                IdxSlot::CatIdxInt(Slot::from(el))
            }
            SetType::CatSetStr => {
                let el = self.cat_idx_str.push(unsafe { empty_string() });
                IdxSlot::CatIdxStr(Slot::from(el))
            }
            SetType::OrdSet => {
                let el = self.ord_idx.push(0);
                IdxSlot::OrdIdx(Slot::from(el))
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
