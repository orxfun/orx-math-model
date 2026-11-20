use crate::sets::{CatInt, CatStr, Ord};
use crate::Dim;

pub trait Dimension {
    type Sets<'m, 'd: 'm>;

    fn dim() -> Dim;
}

// d0

pub struct D0;
impl Dimension for D0 {
    type Sets<'m, 'd: 'm> = AnySets<'m, 'd, 0>;

    fn dim() -> Dim {
        Dim::D0
    }
}

impl<'m, 'd> From<()> for AnySets<'m, 'd, 0> {
    fn from(_: ()) -> Self {
        AnySets([])
    }
}

// d1

pub struct D1;
impl Dimension for D1 {
    type Sets<'m, 'd: 'm> = AnySets<'m, 'd, 1>;

    fn dim() -> Dim {
        Dim::D1
    }
}

macro_rules! any_d1 {
    ($s0:ty) => {
        impl<'m, 'd> From<$s0> for AnySets<'m, 'd, 1> {
            fn from(value: $s0) -> Self {
                Self([value.into()])
            }
        }
    };
}
any_d1!(CatInt<'m, 'd>);
any_d1!(CatStr<'m, 'd>);
any_d1!(Ord<'m, 'd>);

// d2

pub struct D2;
impl Dimension for D2 {
    type Sets<'m, 'd: 'm> = AnySets<'m, 'd, 2>;

    fn dim() -> Dim {
        Dim::D2
    }
}

macro_rules! any_d2 {
    ($s0:ty, $s1:ty) => {
        impl<'m, 'd> From<($s0, $s1)> for AnySets<'m, 'd, 2> {
            fn from((a, b): ($s0, $s1)) -> Self {
                Self([a.into(), b.into()])
            }
        }
    };
}

any_d2!(CatInt<'m, 'd>, CatInt<'m, 'd>);
any_d2!(CatInt<'m, 'd>, CatStr<'m, 'd>);
any_d2!(CatInt<'m, 'd>, Ord<'m, 'd>);

any_d2!(CatStr<'m, 'd>, CatInt<'m, 'd>);
any_d2!(CatStr<'m, 'd>, CatStr<'m, 'd>);
any_d2!(CatStr<'m, 'd>, Ord<'m, 'd>);

any_d2!(Ord<'m, 'd>, CatInt<'m, 'd>);
any_d2!(Ord<'m, 'd>, CatStr<'m, 'd>);
any_d2!(Ord<'m, 'd>, Ord<'m, 'd>);

// any sets

#[derive(derive_more::From)]
pub enum AnySet<'m, 'd> {
    CatInt(CatInt<'m, 'd>),
    CatStr(CatStr<'m, 'd>),
    Ord(Ord<'m, 'd>),
}

pub struct AnySets<'m, 'd, const D: usize>([AnySet<'m, 'd>; D]);

// con

fn con<'m, 'd: 'm, D: Dimension>(sets: impl Into<D::Sets<'m, 'd>>) {
    //
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn abc() {
        let m = Model::new();
        let s0 = m.cat_set_int();
        let s1 = m.cat_set_str();
        let s2 = m.ord_set();

        let c0 = con::<D0>(());
        let c1 = con::<D1>(s0);
        let c2 = con::<D2>((s0, s1));
    }
}
