use crate::elements::Elements;
use crate::index_types::IdxInVariant;
use crate::sets::DSet2;
use alloc::boxed::Box;

pub trait IdxIn<'d>: Sized {
    fn variant() -> IdxInVariant;

    // set

    fn bind_set(set: &mut DSet2<'d>, fun: impl Fn(&mut Elements<Self>) + 'd);

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd);
}

// usize

impl<'d> IdxIn<'d> for usize {
    fn variant() -> IdxInVariant {
        IdxInVariant::Usize
    }

    // set

    fn bind_set(set: &mut DSet2<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        match set {
            DSet2::CatUsize(_) | DSet2::CatStr(_) => *set = DSet2::CatUsize(Box::new(fun)),
            _ => unreachable!(),
        }
    }

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd) {
    //     data.fun_int = Box::new(fun);
    // }
}

// &str

impl<'d> IdxIn<'d> for &'d str {
    fn variant() -> IdxInVariant {
        IdxInVariant::Str
    }

    // set

    fn bind_set(set: &mut DSet2<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        match set {
            DSet2::CatUsize(_) | DSet2::CatStr(_) => *set = DSet2::CatStr(Box::new(fun)),
            _ => unreachable!(),
        }
    }
}

// i64

impl<'d> IdxIn<'d> for i64 {
    fn variant() -> IdxInVariant {
        IdxInVariant::I64
    }

    // set

    fn bind_set(set: &mut DSet2<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        match set {
            DSet2::Ord(x) => *x = Box::new(fun),
            _ => unreachable!(),
        }
    }
}
