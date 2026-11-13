use crate::{Res, ResValid};
use alloc::string::String;
use core::mem::ManuallyDrop;

pub trait TConVal {
    fn typ() -> ConValTyp;

    fn into_union(self) -> UConVal;

    fn from_union(u: &UConVal) -> &Self;
}

impl TConVal for String {
    fn typ() -> ConValTyp {
        ConValTyp::String
    }

    fn into_union(self) -> UConVal {
        UConVal {
            string: ManuallyDrop::new(self),
        }
    }

    fn from_union(u: &UConVal) -> &Self {
        unsafe { &u.string }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConValTyp {
    String,
}

#[repr(C)]
pub union UConVal {
    pub string: ManuallyDrop<String>,
}

pub struct ConVal {
    pub typ: ConValTyp,
    data: UConVal,
}

impl Drop for ConVal {
    fn drop(&mut self) {
        unsafe {
            match self.typ {
                ConValTyp::String => ManuallyDrop::drop(&mut self.data.string),
            }
        }
    }
}

impl ConVal {
    pub fn new<T: TConVal>(value: T) -> Self {
        let typ = T::typ();
        let data = T::into_union(value);
        Self { typ, data }
    }

    pub fn read<T: TConVal>(&self) -> &T {
        Res::dbg_assert_same_con_val(T::typ(), self.typ);
        T::from_union(&self.data)
    }
}
