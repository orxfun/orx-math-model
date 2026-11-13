use crate::{MetaSet, Res, SetTyp, Validation};
use alloc::string::String;
use core::mem::transmute;

#[repr(C)]
#[derive(Clone, Copy)]
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
    pub(super) fn new(typ: SetTyp) -> Self {
        let data = match typ {
            SetTyp::CatInt => UIdxVal { cat_int: 0 },
            SetTyp::CatStr => UIdxVal { cat_str: str() },
            SetTyp::Ord => UIdxVal { ord: 0 },
        };
        Self { typ, data }
    }

    pub fn get<S: MetaSet>(&self) -> S::IdxRaw<'d> {
        Res::dbg_assert_same_set(S::typ(), self.typ);
        S::idx_unpack_val(self.data)
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
