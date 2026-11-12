use crate::binding::Binding;
use crate::elements::{Elements, ElementsIter};
use crate::index_types::IdxInVariant;
use crate::indices::DIdx;
use crate::pars::{DParD1, FParD1};
use crate::sets::DSet;
use alloc::boxed::Box;
use alloc::string::String;

pub trait IdxIn<'d>: Sized + Clone + Copy + Into<IdxInVariant<'d>> {
    // set

    fn set_bind(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd);

    fn set_fill(set: &DSet<'d>, elements: &mut Elements<Self>);

    // idx

    fn idx_iter<'a>(idx: &DIdx<'d>, elements: &'a mut Elements<Self>) -> ElementsIter<'a, Self>;
}

// usize

impl From<usize> for IdxInVariant<'_> {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl<'d> IdxIn<'d> for usize {
    // set

    fn set_bind(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        assert!(!set.is_bound());
        match set {
            DSet::CatUsize(_, _) => *set = DSet::CatUsize(Binding::Bound, Box::new(fun)),
            _ => unreachable!(),
        }
    }

    fn set_fill(set: &DSet<'d>, elements: &mut Elements<Self>) {
        assert!(set.is_bound());
        match set {
            DSet::CatUsize(_, f) => f(elements),
            _ => unreachable!(),
        }
    }

    // idx

    fn idx_iter<'a>(idx: &DIdx<'d>, elements: &'a mut Elements<Self>) -> ElementsIter<'a, Self> {
        idx.set().fill(elements);
        match idx {
            DIdx::CatUsize(_, v) => ElementsIter::new(elements.iter(), *v),
            _ => unreachable!(),
        }
    }
}

// &String

impl<'d> From<&'d String> for IdxInVariant<'d> {
    fn from(value: &'d String) -> Self {
        Self::Str(value)
    }
}

impl<'d> IdxIn<'d> for &'d String {
    // set

    fn set_bind(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        assert!(!set.is_bound());
        match set {
            DSet::CatStr(_, _) => *set = DSet::CatStr(Binding::Bound, Box::new(fun)),
            _ => unreachable!(),
        }
    }

    fn set_fill(set: &DSet<'d>, elements: &mut Elements<Self>) {
        assert!(set.is_bound());
        match set {
            DSet::CatStr(_, f) => f(elements),
            _ => unreachable!(),
        }
    }

    // idx

    fn idx_iter<'a>(idx: &DIdx<'d>, elements: &'a mut Elements<Self>) -> ElementsIter<'a, Self> {
        idx.set().fill(elements);
        match idx {
            DIdx::CatStr(_, v) => ElementsIter::new(elements.iter(), *v),
            _ => unreachable!(),
        }
    }
}

// i64

impl From<i64> for IdxInVariant<'_> {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl<'d> IdxIn<'d> for i64 {
    // set

    fn set_bind(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        assert!(!set.is_bound());
        match set {
            DSet::Ord(_, _) => *set = DSet::Ord(Binding::Bound, Box::new(fun)),
            _ => unreachable!(),
        }
    }

    fn set_fill(set: &DSet<'d>, elements: &mut Elements<Self>) {
        assert!(set.is_bound());
        match set {
            DSet::Ord(_, f) => f(elements),
            _ => unreachable!(),
        }
    }

    // idx

    fn idx_iter<'a>(idx: &DIdx<'d>, elements: &'a mut Elements<Self>) -> ElementsIter<'a, Self> {
        idx.set().fill(elements);
        match idx {
            DIdx::Ord(_, v) => ElementsIter::new(elements.iter(), *v),
            _ => unreachable!(),
        }
    }
}
