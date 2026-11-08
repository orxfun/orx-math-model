use crate::index_types::IdxInVariant;

pub trait IdxIn<'d>: Sized {
    fn variant() -> IdxInVariant;

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd);
}

// usize

impl<'d> IdxIn<'d> for usize {
    fn variant() -> IdxInVariant {
        IdxInVariant::USize
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

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd) {
    //     data.fun_str = Box::new(fun);
    // }
}

// i64

impl<'d> IdxIn<'d> for i64 {
    fn variant() -> IdxInVariant {
        IdxInVariant::I64
    }

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd) {
    //     data.fun_int = Box::new(fun);
    // }
}
