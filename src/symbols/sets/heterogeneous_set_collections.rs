use crate::{symbols::sets::SetCore, Set};

pub struct SetCore1<'m> {
    pub s1: SetCore<'m>,
}

pub struct SetCore2<'m> {
    pub s1: SetCore<'m>,
    pub s2: SetCore<'m>,
}

pub struct SetCore3<'m> {
    pub s1: SetCore<'m>,
    pub s2: SetCore<'m>,
    pub s3: SetCore<'m>,
}

pub struct SetCore4<'m> {
    pub s1: SetCore<'m>,
    pub s2: SetCore<'m>,
    pub s3: SetCore<'m>,
    pub s4: SetCore<'m>,
}

// tuple to array conversions

impl<'m, const N: usize> From<Set<'m, N>> for SetCore1<'m> {
    fn from(s1: Set<'m, N>) -> Self {
        Self { s1: s1.into() }
    }
}

impl<'m, const N: usize> From<(Set<'m, N>, Set<'m, N>)> for SetCore2<'m> {
    fn from((s1, s2): (Set<'m, N>, Set<'m, N>)) -> Self {
        Self {
            s1: s1.into(),
            s2: s2.into(),
        }
    }
}

impl<'m, const N: usize> From<(Set<'m, N>, Set<'m, N>, Set<'m, N>)> for SetCore3<'m> {
    fn from((s1, s2, s3): (Set<'m, N>, Set<'m, N>, Set<'m, N>)) -> Self {
        Self {
            s1: s1.into(),
            s2: s2.into(),
            s3: s3.into(),
        }
    }
}

impl<'m, const N: usize> From<(Set<'m, N>, Set<'m, N>, Set<'m, N>, Set<'m, N>)> for SetCore4<'m> {
    fn from((s1, s2, s3, s4): (Set<'m, N>, Set<'m, N>, Set<'m, N>, Set<'m, N>)) -> Self {
        Self {
            s1: s1.into(),
            s2: s2.into(),
            s3: s3.into(),
            s4: s4.into(),
        }
    }
}
