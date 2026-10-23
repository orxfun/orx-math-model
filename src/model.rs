use crate::symbols::{Idx, IdxData, Par, ParData, Sca, ScaFun, Set, SetData, SetFun, SetRef};
use crate::symbols_d1::{ParD1, ParD1Data, ParD1Fun, SetD1, SetD1Data, SetD1Fun};
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
    par_d1: Bag<ParD1Data<'d>>,

    idx: Bag<IdxData>,

    sca: Bag<ScaFun<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set: new_bag(),
            set_d1: new_bag(),

            par: new_bag(),
            par_d1: new_bag(),

            idx: new_bag(),

            sca: new_bag(),
        }
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        self.add_set(SetData::new(SetFun::empty()))
    }

    pub(crate) fn add_set(&self, data: SetData<'d>) -> Set<'_, 'd> {
        Set::new(self, self.set.push(data))
    }

    pub fn set_d1<'m>(&'m self, set: Set<'m, 'd>) -> SetD1<'m, 'd> {
        let data = SetD1Data::new(self.set_ref(set), SetD1Fun::empty());
        SetD1::new(self, self.set_d1.push(data))
    }

    // pars

    pub fn par(&self) -> Par<'_, 'd> {
        let data = ParData::new(0.0);
        Par::new(self, self.par.push(data))
    }

    pub fn par_d1<'m>(&'m self, set: Set<'m, 'd>) -> ParD1<'m, 'd> {
        let data = ParD1Data::new(self.set_ref(set), ParD1Fun::empty());
        ParD1::new(self, self.par_d1.push(data))
    }

    // idx

    pub(crate) fn idx<'m>(&'m self, set: Set<'m, 'd>) -> Idx<'m, 'd> {
        let data = IdxData::new(self.set_ref(set), 0);
        Idx::new(self, self.idx.push(data))
    }

    // scalars

    pub(crate) fn add_sca<'m>(&'m self, fun: ScaFun<'d>) -> Sca<'m, 'd> {
        Sca::new(self, self.sca.push(fun))
    }

    // helper

    fn set_ref(&self, set: Set<'_, 'd>) -> SetRef {
        let set_pos = set.el.position_in(&self.set).expect("TODO: error msg");
        SetRef::new(set_pos)
    }
}
