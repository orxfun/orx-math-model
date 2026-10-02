use crate::symbols::symbol_data::SymbolData;
use orx_imp_vec::ImpVec;

pub struct Model {
    num_sets: ImpVec<SymbolData>,
    cat_sets: ImpVec<SymbolData>,
}
