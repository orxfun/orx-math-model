use crate::symbols::{definition::Definition, symbol::Symbol};
use alloc::string::String;

pub struct SymbolData<S: Symbol> {
    pub key: String,
    pub definition: Definition,
    pub data: S::Data,
}
