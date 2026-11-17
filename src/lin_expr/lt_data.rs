use crate::scalars::DSca;
use crate::Ptr;

#[derive(derive_new::new, Clone)]
pub struct DLinTerm<'d> {
    coef: Option<Ptr<'d, DSca<'d>>>,
    var: Ptr<'d, DSca<'d>>,
}
