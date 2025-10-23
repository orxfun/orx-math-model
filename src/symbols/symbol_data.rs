use crate::symbols::definition::Definition;
use alloc::string::String;

pub struct SymbolData<Data> {
    pub key: String,
    pub definition: Definition,
    pub data: Data,
}
