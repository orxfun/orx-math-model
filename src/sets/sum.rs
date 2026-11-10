use orx_imp_vec::Elem;

use crate::sets::{CatSet, DSet, OrdSet};

pub enum Set<'m, 'd> {
    CatUsize(CatSet<'m, 'd>),
    CatStr(CatSet<'m, 'd>),
    Ord(OrdSet<'m, 'd>),
}

impl<'m, 'd> From<CatSet<'m, 'd>> for Set<'m, 'd> {
    fn from(value: CatSet<'m, 'd>) -> Self {
        match unsafe { value.el.get() } {
            DSet::CatUsize(_, _) => Self::CatUsize(value),
            DSet::CatStr(_, _) => Self::CatStr(value),
            _ => unreachable!(),
        }
    }
}

impl<'m, 'd> From<OrdSet<'m, 'd>> for Set<'m, 'd> {
    fn from(value: OrdSet<'m, 'd>) -> Self {
        match unsafe { value.el.get() } {
            DSet::Ord(_, _) => Self::Ord(value),
            _ => unreachable!(),
        }
    }
}

impl<'m, 'd> Set<'m, 'd> {
    pub fn el(&self) -> Elem<'_, DSet<'d>> {
        match self {
            Self::CatUsize(x) => x.el,
            Self::CatStr(x) => x.el,
            Self::Ord(x) => x.el,
        }
    }
}
