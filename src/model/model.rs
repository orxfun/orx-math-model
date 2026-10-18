use crate::model::symbols::{Par, Set};
use crate::model::{Set0Gen, Slots};
use orx_imp_vec::ImpVec;

pub struct Model<'d> {
    int: ImpVec<i64>,
    set0b: Slots<Set0Gen<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            int: ImpVec::new(),
            set0b: Slots::new(),
        }
    }

    // pars

    pub fn par_d0(&self) -> Par<'_> {
        Par::new(self.int.push(0))
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        Set::new(self.set0b.push_default())
    }
}
