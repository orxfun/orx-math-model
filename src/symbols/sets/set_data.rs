use alloc::string::String;

pub struct SetData {
    key: String,
}

impl SetData {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
