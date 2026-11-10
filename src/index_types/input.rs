use crate::binding::Binding;
use crate::elements::Elements;
use crate::index_types::IdxInVariant;
use crate::sets::DSet;
use alloc::boxed::Box;

pub trait IdxIn<'d>: Sized {
    fn variant() -> IdxInVariant;

    // set

    fn bind_set(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd);

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd);
}

// usize

impl<'d> IdxIn<'d> for usize {
    fn variant() -> IdxInVariant {
        IdxInVariant::Usize
    }

    // set

    fn bind_set(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        assert!(!set.is_bound());
        match set {
            DSet::CatUsize(_, _) | DSet::CatStr(_, _) => {
                *set = DSet::CatUsize(Binding::Bound, Box::new(fun))
            }
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

    fn bind_set(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        assert!(!set.is_bound());
        match set {
            DSet::CatUsize(_, _) | DSet::CatStr(_, _) => {
                *set = DSet::CatStr(Binding::Bound, Box::new(fun))
            }
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

    fn bind_set(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        assert!(!set.is_bound());
        match set {
            DSet::Ord(_, x) => *x = Box::new(fun),
            _ => unreachable!(),
        }
    }
}
