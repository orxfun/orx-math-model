use crate::bag::{new_bag, Bag};
use crate::indices::DIdx;
use crate::sets::{CatSet, DSet, OrdSet};
use orx_imp_vec::Elem;

pub struct Model<'d> {
    sets: Bag<DSet<'d>>,
    indices: Bag<DIdx<'d>>,

    // values
    val_usize: Bag<usize>,
    val_str: Bag<&'d str>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            sets: new_bag(),
            indices: new_bag(),

            // values
            val_usize: new_bag(),
            val_str: new_bag(),
        }
    }

    // sets

    pub fn set_cat_int<'m>(&'m self) -> CatSet<'m, 'd, usize> {
        CatSet::new(self, self.sets.push(DSet::empty_cat_int()))
    }

    pub fn set_cat_str<'m>(&'m self) -> CatSet<'m, 'd, &'d str> {
        CatSet::new(self, self.sets.push(DSet::empty_cat_str()))
    }

    pub fn set_ord<'m>(&'m self) -> OrdSet<'m, 'd> {
        OrdSet::new(self, self.sets.push(DSet::empty_ord()))
    }

    // indices

    pub(crate) fn index<'m>(&'m self, data: DIdx<'d>) -> Elem<'m, DIdx<'d>> {
        self.indices.push(data)
    }

    // values

    pub(crate) fn val_usize<'m>(&'m self) -> Elem<'m, usize> {
        self.val_usize.push(Default::default())
    }

    pub(crate) fn val_str<'m>(&'m self) -> Elem<'m, &'d str> {
        self.val_str.push(Default::default())
    }
}
