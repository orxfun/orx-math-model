use crate::{
    data_structs::forall_set_refs::ForAllSetRefs,
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{data::forall::ForAll, set::Set},
    },
};

pub trait ToForAll {
    fn to_forall(&self) -> ForAll<Building>;
}

impl ToForAll for Set<Building> {
    fn to_forall(&self) -> ForAll<Building> {
        ForAll {
            sets: ForAllSetRefs::new([*self.sym_ref()].into()),
        }
    }
}
impl ToForAll for (Set<Building>, Set<Building>) {
    fn to_forall(&self) -> ForAll<Building> {
        ForAll {
            sets: ForAllSetRefs::new([*self.0.sym_ref(), *self.1.sym_ref()].into()),
        }
    }
}
impl ToForAll for (Set<Building>, Set<Building>, Set<Building>) {
    fn to_forall(&self) -> ForAll<Building> {
        ForAll {
            sets: ForAllSetRefs::new(
                [*self.0.sym_ref(), *self.1.sym_ref(), *self.2.sym_ref()].into(),
            ),
        }
    }
}
impl ToForAll for (Set<Building>, Set<Building>, Set<Building>, Set<Building>) {
    fn to_forall(&self) -> ForAll<Building> {
        ForAll {
            sets: ForAllSetRefs::new(
                [
                    *self.0.sym_ref(),
                    *self.1.sym_ref(),
                    *self.2.sym_ref(),
                    *self.3.sym_ref(),
                ]
                .into(),
            ),
        }
    }
}
