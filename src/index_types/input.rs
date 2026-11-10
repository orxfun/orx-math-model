use crate::binding::Binding;
use crate::elements::{Elements, ElementsIter};
use crate::index_types::IdxInVariant;
use crate::indices::DIdx;
use crate::sets::DSet;
use alloc::boxed::Box;

pub trait IdxIn<'d>: Sized + Clone + Copy {
    fn into_variant(self) -> IdxInVariant<'d>;

    // set

    fn set_bind<'m>(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd);

    fn set_fill(set: &DSet<'d>, elements: &mut Elements<Self>);

    // idx

    fn idx_iter<'a>(idx: &DIdx<'d>, elements: &'a mut Elements<Self>) -> ElementsIter<'a, Self>;

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd);
}

// usize

impl<'d> IdxIn<'d> for usize {
    fn into_variant(self) -> IdxInVariant<'d> {
        IdxInVariant::Usize(self)
    }

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
            _ => todo!(),
        }
    }

    // fn update_dpar_d1<T>(data: &mut DParD1<'d, T>, fun: impl Fn(Self) -> T + 'd) {
    //     data.fun_int = Box::new(fun);
    // }
}

// &str

impl<'d> IdxIn<'d> for &'d str {
    fn into_variant(self) -> IdxInVariant<'d> {
        IdxInVariant::Str(self)
    }

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
            _ => todo!(),
        }
    }
}

// i64

impl<'d> IdxIn<'d> for i64 {
    fn into_variant(self) -> IdxInVariant<'d> {
        IdxInVariant::I64(self)
    }

    // set

    fn set_bind(set: &mut DSet<'d>, fun: impl Fn(&mut Elements<Self>) + 'd) {
        assert!(!set.is_bound());
        match set {
            DSet::Ord(_, x) => *set = DSet::Ord(Binding::Bound, Box::new(fun)),
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
            _ => todo!(),
        }
    }
}
