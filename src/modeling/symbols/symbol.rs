#[derive(Clone, Debug)]
pub struct Symbol {
    pub(crate) key: String,
    pub(crate) definition: Option<String>,
}

impl Symbol {
    pub(crate) fn new(key: String, definition: Option<String>) -> Self {
        Self { key, definition }
    }
    pub(crate) fn into_key_definition(self) -> (String, String) {
        (self.key, self.definition.unwrap_or(String::from("")))
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self::new(value.to_string(), None)
    }
}
impl From<(&str, &str)> for Symbol {
    fn from(value: (&str, &str)) -> Self {
        Self::new(value.0.to_string(), Some(value.1.to_string()))
    }
}
