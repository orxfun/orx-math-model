#[derive(derive_new::new)]
pub struct ParD0<'a> {
    value: &'a i64,
}

impl ParD0<'_> {
    pub fn value(&self) -> i64 {
        *self.value
    }
}
