use alloc::string::String;

#[derive(derive_new::new, Default, Debug)]
pub struct SymbolData {
    key: Option<String>,
    description: Option<String>,
}
