use crate::{scalars::DVar, Ptr};

#[derive(derive_new::new, Clone)]
pub struct DLinTerm<'d> {
    coef: Option<Ptr<'d, DVar<'d>>>,
    var: Ptr<'d, DVar<'d>>,
}
