use crate::Model;
use orx_imp_vec::Elem;

//data

#[derive(derive_new::new)]
pub struct ParData(f64);

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Par<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParData>,
}
