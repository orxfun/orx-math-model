use crate::elements::{Elements, ElementsIter};
use crate::indices::idx::DIdx2;
use crate::Model;
use core::marker::PhantomData;
use core::{iter::Copied, slice::Iter};
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct CatIdx2<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DIdx2<'d>>,
}
