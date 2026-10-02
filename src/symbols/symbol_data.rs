use alloc::string::String;

#[derive(derive_new::new, Debug)]
pub struct SymbolData {
    key: String,
    description: Option<String>,
}
