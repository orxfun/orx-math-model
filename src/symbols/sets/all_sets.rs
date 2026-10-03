use crate::symbols::sets::{CatSet, CatSetData, RangeSet, RangeSetData};
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::{Model, Symbol};
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub struct Sets {
    cat: ImpVec<CatSetData>,
    range: ImpVec<RangeSetData>,

    // run data
    cat_run_idx: ImpVec<usize>,
    range_run_idx: ImpVec<usize>,
}

impl Sets {
    pub fn add_cat<'a>(&'a self, m: &'a Model) -> CatSet<'a> {
        self.cat_run_idx.imp_push(0);

        let data = CatSetData::new(Default::default());
        let d = self.cat.imp_push_get_ref(data);

        CatSet::new(m, d)
    }

    // pub fn add_range<'a>(&'a self, m: &'a Model) -> RangeSet<'a> {
    //     self.range_run_idx.imp_push(0);
    //     let s = Symbol::new(m, self.cat.imp_push_get_ref(Default::default()));
    //     RangeSet::new(s)
    // }
}
