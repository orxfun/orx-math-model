use crate::indices::idx::DIdx;
use crate::Model;
use core::marker::PhantomData;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
#[new(visibility = "pub(crate)")]
pub struct CatIdx<'m, 'd, K: Copy + Default> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DIdx<'d>>,
    p: PhantomData<K>,
}
