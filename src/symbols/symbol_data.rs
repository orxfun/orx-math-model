use crate::symbols::{definition::Definition, symbol_definition::SymbolDef};
use alloc::string::String;

pub struct SymbolData<S: SymbolDef> {
    pub key: String,
    pub definition: Definition,
    pub data: S::Data,
}
