use alloc::string::String;

#[derive(derive_new::new)]
pub struct SymbolData {
    key: String,
    description: Option<String>,
}
