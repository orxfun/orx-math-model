use super::symbol::Symbol;
use orx_imp_vec::ImpVec;

pub struct SymbolCollection<S: Symbol> {
    storage: ImpVec<S>,
}
