use alloc::string::String;

#[derive(derive_new::new, Default, Debug)]
pub struct SymbolDefinition {
    key: Option<String>,
    description: Option<String>,
}
