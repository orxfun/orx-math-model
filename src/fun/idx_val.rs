use crate::{MetaSet, SetTyp};
use alloc::string::String;
use core::mem::transmute;

pub enum IdxVal<'d> {
    CatInt(usize),
    CatStr(&'d String),
    Ord(i64),
}

impl<'d> IdxVal<'d> {
    pub(super) fn new(typ: SetTyp) -> Self {
        match typ {
            SetTyp::CatInt => Self::CatInt(0),
            SetTyp::CatStr => Self::CatStr(empty_string()),
            SetTyp::Ord => Self::Ord(0),
        }
    }

    pub fn get<S: MetaSet>(&self) -> &S::IdxRaw<'d> {
        match (S::typ(), self) {
            (SetTyp::CatInt, Self::CatInt(f)) => unsafe { transmute(f) },
            (SetTyp::CatStr, Self::CatStr(f)) => unsafe { transmute(f) },
            (SetTyp::Ord, Self::Ord(f)) => unsafe { transmute(f) },
            _ => unreachable!("invalid FunE::set call"),
        }
    }
}

fn empty_string() -> &'static String {
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
