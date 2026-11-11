use crate::pars::{DParD1, FParD1};
use crate::sets::DSet;
use crate::{binding::Binding, index_types::IdxIn, values::Value};
use alloc::boxed::Box;
use alloc::string::String;

pub trait Io1<'d> {
    type I0: IdxIn<'d>;

    type O: Value;

    fn p1_bind(par: &mut DParD1<'d>, fun: impl Fn(Self::I0) -> Self::O + 'd);
}

// I = usize

impl<'d> Io1<'d> for (usize, i64) {
    type I0 = usize;

    type O = i64;

    fn p1_bind(par: &mut DParD1<'d>, fun: impl Fn(Self::I0) -> Self::O + 'd) {
        assert!(!par.is_bound());
        match par {
            DParD1::Int(_, s, _) => match s.val() {
                DSet::CatUsize(_, _) => {
                    *par = DParD1::Int(Binding::Bound, *s, FParD1::CatUsize(Box::new(fun)))
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl<'d> Io1<'d> for (usize, f64) {
    type I0 = usize;

    type O = f64;

    fn p1_bind(par: &mut DParD1<'d>, fun: impl Fn(Self::I0) -> Self::O + 'd) {
        assert!(!par.is_bound());
        match par {
            DParD1::Num(_, s, _) => match s.val() {
                DSet::CatUsize(_, _) => {
                    *par = DParD1::Num(Binding::Bound, *s, FParD1::CatUsize(Box::new(fun)))
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

// I = &str

impl<'d> Io1<'d> for (&'d String, i64) {
    type I0 = &'d String;

    type O = i64;

    fn p1_bind(par: &mut DParD1<'d>, fun: impl Fn(Self::I0) -> Self::O + 'd) {
        assert!(!par.is_bound());
        match par {
            DParD1::Int(_, s, _) => match s.val() {
                DSet::CatStr(_, _) => {
                    *par = DParD1::Int(Binding::Bound, *s, FParD1::CatStr(Box::new(fun)))
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl<'d> Io1<'d> for (&'d String, f64) {
    type I0 = &'d String;

    type O = f64;

    fn p1_bind(par: &mut DParD1<'d>, fun: impl Fn(Self::I0) -> Self::O + 'd) {
        assert!(!par.is_bound());
        match par {
            DParD1::Num(_, s, _) => match s.val() {
                DSet::CatStr(_, _) => {
                    *par = DParD1::Num(Binding::Bound, *s, FParD1::CatStr(Box::new(fun)))
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

// I = i64

impl<'d> Io1<'d> for (i64, i64) {
    type I0 = i64;

    type O = i64;

    fn p1_bind(par: &mut DParD1<'d>, fun: impl Fn(Self::I0) -> Self::O + 'd) {
        assert!(!par.is_bound());
        match par {
            DParD1::Int(_, s, _) => match s.val() {
                DSet::Ord(_, _) => {
                    *par = DParD1::Int(Binding::Bound, *s, FParD1::Ord(Box::new(fun)))
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl<'d> Io1<'d> for (i64, f64) {
    type I0 = i64;

    type O = f64;

    fn p1_bind(par: &mut DParD1<'d>, fun: impl Fn(Self::I0) -> Self::O + 'd) {
        assert!(!par.is_bound());
        match par {
            DParD1::Num(_, s, _) => match s.val() {
                DSet::Ord(_, _) => {
                    *par = DParD1::Num(Binding::Bound, *s, FParD1::Ord(Box::new(fun)))
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
