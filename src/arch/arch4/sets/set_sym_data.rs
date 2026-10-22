use alloc::string::String;
use orx_concurrent_option::ConcurrentOption;

#[derive(Default)]
pub struct SetSymData {
    pub key: ConcurrentOption<String>,
    pub definition: ConcurrentOption<String>,
}

impl SetSymData {
    pub fn set_key(&self, value: impl Into<String>) {
        self.key.set_some(value.into());
    }

    pub fn set_definition(&self, value: impl Into<String>) {
        self.definition.set_some(value.into());
    }
}
