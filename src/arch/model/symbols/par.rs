use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct Par<'m> {
    elem: Elem<'m, i64>,
}

impl Par<'_> {
    pub(crate) fn value(&self) -> i64 {
        self.elem.copied()
    }

    pub(crate) fn set(&self, value: i64) {
        self.elem.set(value);
    }
}
