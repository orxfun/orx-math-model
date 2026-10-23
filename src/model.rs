use crate::model_references::SetRef;
use crate::symbols::{Elements, Idx, IdxData, Set, SetData};
use crate::symbols_d1::{SetD1, SetD1Data};
use alloc::boxed::Box;
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;
type Bag<T> = ImpVec<T, Pinned<T>>;
fn new_bag<T>() -> Bag<T> {
    Bag::new()
}

pub struct Model<'d> {
    pub(crate) set: Bag<SetData<'d>>,
    pub(crate) set_d1: Bag<SetD1Data<'d>>,

    pub(crate) idx: Bag<IdxData>,

    indices: Bag<usize>,
    set_d1_gen: Bag<Box<dyn Fn(usize, &mut Elements) + 'd>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set: new_bag(),
            set_d1: new_bag(),

            idx: new_bag(),

            indices: new_bag(),
            set_d1_gen: new_bag(),
        }
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        self.add_set(SetData::default())
    }

    pub(crate) fn add_set<'m>(&'m self, data: SetData<'d>) -> Set<'m, 'd> {
        Set::new(self, self.set.push(data))
    }

    pub fn set_d1<'m>(&'m self, set: Set<'m, 'd>) -> SetD1<'m, 'd> {
        let fun_ptr = self.set_d1_gen.push(Box::new(|_, _| {})).as_mut_ptr();
        let set_ref = SetRef::new(set);
        let data = SetD1Data::new(set_ref, fun_ptr);
        SetD1::new(self, self.set_d1.push(data))
    }

    // idx

    pub(crate) fn idx<'m>(&'m self, set: Set<'m, 'd>) -> Idx<'m, 'd> {
        let set = SetRef::new(set);
        let value_ptr = self.indices.push(0).as_mut_ptr();
        let data = IdxData::new(set, value_ptr);
        Idx::new(self, self.idx.push(data))
    }
}
