use crate::{Set, Res, ResValid, SetTyp};
use alloc::string::String;

#[repr(C)]
pub union UIdxVal<'d> {
    pub cat_int: usize,
    pub cat_str: &'d String,
    pub ord: i64,
}

pub struct IdxVal<'d> {
    typ: SetTyp,
    data: UIdxVal<'d>,
}

impl<'d> IdxVal<'d> {
    pub fn empty(typ: SetTyp) -> Self {
        let data = match typ {
            SetTyp::CatInt => UIdxVal { cat_int: 0 },
            SetTyp::CatStr => UIdxVal { cat_str: str() },
            SetTyp::Ord => UIdxVal { ord: 0 },
        };
        Self { typ, data }
    }

    pub fn read<S: Set>(&self) -> S::IdxRaw<'d> {
        Res::dbg_assert_same_set(S::typ(), self.typ);
        S::idx_unpack(&self.data)
    }

    pub fn write<S: Set>(&mut self, value: S::IdxRaw<'d>) {
        Res::dbg_assert_same_set(S::typ(), self.typ);
        S::idx_write(&mut self.data, value);
    }
}

fn str() -> &'static String {
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicBool, Ordering};

    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    static mut STRING: MaybeUninit<String> = MaybeUninit::uninit();

    if !INITIALIZED.load(Ordering::Acquire) {
        unsafe { STRING.write(Default::default()) };
        INITIALIZED.store(true, Ordering::Release);
    }
    unsafe { STRING.assume_init_ref() }
}
