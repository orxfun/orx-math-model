use alloc::string::String;

#[derive(derive_new::new, Default, Debug, Clone)]
pub struct SymbolDefinition {
    key: Option<String>,
    description: Option<String>,
}
