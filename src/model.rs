use crate::symbols::{Idx, IdxData, Par, ParData, Sca, ScaFun, Set, SetData, SetFun, SetRef};
use crate::symbols_d1::{SetD1, SetD1Data, SetD1Fun};
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;
type Bag<T> = ImpVec<T, Pinned<T>>;
fn new_bag<T>() -> Bag<T> {
    Bag::new()
}

pub struct Model<'d> {
    set: Bag<SetData<'d>>,
    set_d1: Bag<SetD1Data<'d>>,

    par: Bag<ParData>,

    idx: Bag<IdxData>,

    // data
    indices: Bag<usize>,
    set_d0_gen: Bag<SetFun<'d>>,
    set_d1_gen: Bag<SetD1Fun<'d>>,
    sca_gen: Bag<ScaFun<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set: new_bag(),
            set_d1: new_bag(),

            par: new_bag(),

            idx: new_bag(),

            indices: new_bag(),
            set_d0_gen: new_bag(),
            set_d1_gen: new_bag(),
            sca_gen: new_bag(),
        }
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        self.add_set(SetFun::empty())
    }

    pub(crate) fn add_set(&self, fun: SetFun<'d>) -> Set<'_, 'd> {
        let fun_ptr = self.set_d0_gen.push(fun).as_mut_ptr();
        let data = SetData::new(fun_ptr);
        Set::new(self, self.set.push(data))
    }

    pub fn set_d1<'m>(&'m self, set: Set<'m, 'd>) -> SetD1<'m, 'd> {
        let fun_ptr = self.set_d1_gen.push(SetD1Fun::empty()).as_mut_ptr();
        let data = SetD1Data::new(self.set_ref(set), fun_ptr);
        SetD1::new(self, self.set_d1.push(data))
    }

    // pars

    pub fn par(&self) -> Par<'_, 'd> {
        let data = ParData::new(0.0);
        Par::new(self, self.par.push(data))
    }

    // idx

    pub(crate) fn idx<'m>(&'m self, set: Set<'m, 'd>) -> Idx<'m, 'd> {
        let value_ptr = self.indices.push(0).as_mut_ptr();
        let data = IdxData::new(self.set_ref(set), value_ptr);
        Idx::new(self, self.idx.push(data))
    }

    // scalars

    pub(crate) fn add_sca<'m>(&'m self, fun: ScaFun<'d>) -> Sca<'m, 'd> {
        Sca::new(self, self.sca_gen.push(fun))
    }

    // helper

    fn set_ref(&self, set: Set<'_, 'd>) -> SetRef {
        let set_pos = set.el.position_in(&self.set).expect("TODO: error msg");
        SetRef::new(set_pos)
    }
}
