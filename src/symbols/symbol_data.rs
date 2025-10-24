use crate::symbols::{definition::Definition, symbol::Symbol};
use alloc::string::String;

pub struct SymbolData<S: Symbol> {
    pub key: String,
    pub definition: Definition,
    // TODO: temporary clippy fix until we use the data
    #[allow(dead_code)]
    pub data: S::Data,
}
