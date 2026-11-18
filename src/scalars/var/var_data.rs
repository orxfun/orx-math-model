use crate::scalars::{FVarIdx, FVarIdxCtor, FVarWrite, FVarWriteCtor};
use crate::vars::DVars;
use crate::{Idx1, Idx2, Ptr, Set};

#[derive(derive_new::new)]
pub struct DVar<'d> {
    parent: Ptr<'d, DVars<'d>>,
    fun_idx: FVarIdx<'d>,
    fun_write: FVarWrite<'d>,
}

impl<'d> DVar<'d> {
    // create

    pub fn create_d1<I0: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx1<'d, I0> + 'd + Copy,
    ) -> Self {
        let i0 = parent.get().set_ptr(0);
        debug_assert_eq!(I0::typ(), i0.get().typ);
        let fun_idx = FVarIdxCtor::d1::<I0>(fun);
        let fun_write = FVarWriteCtor::d1::<I0>(parent, fun);
        Self::new(parent, fun_idx, fun_write)
    }

    pub fn create_d2<I0: Set, I1: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx2<'d, I0, I1> + 'd + Copy,
    ) -> Self {
        let [i0, i1] = [0, 1].map(|i| parent.get().set_ptr(i));
        debug_assert_eq!(I0::typ(), i0.get().typ);
        debug_assert_eq!(I1::typ(), i1.get().typ);
        let fun_idx = FVarIdxCtor::d2::<I0, I1>(fun);
        let fun_write = FVarWriteCtor::d2::<I0, I1>(parent, fun);
        Self::new(parent, fun_idx, fun_write)
    }
}
