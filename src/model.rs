use crate::sets::{Set1, Set2, SetSymData};
use crate::symbols::SymbolRef;
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub struct Model {
    sets: ImpVec<SetSymData>,
}

impl Model {
    pub fn set1<'m, T1>(&'m self) -> Set1<'m, T1> {
        let data_ref = self.sets.imp_push_get_ref(Default::default());
        let symbol_ref = SymbolRef {
            data_ref,
            model_ref: self,
        };
        Set1::new(symbol_ref)
    }

    pub fn set2<'m, T1, T2>(&'m self) -> Set2<'m, T1, T2> {
        let data_ref = self.sets.imp_push_get_ref(Default::default());
        let symbol_ref = SymbolRef {
            data_ref,
            model_ref: self,
        };
        Set2::new(symbol_ref)
    }
}
