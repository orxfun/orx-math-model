use crate::symbols::sets::CatSet;
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::{Model, Symbol};
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllPars {
    count: ImpVec<SymbolDefinition>,
    par1: ImpVec<SymbolDefinition>,
    int_par1: ImpVec<SymbolDefinition>,
}

impl AllPars {
    //
}
