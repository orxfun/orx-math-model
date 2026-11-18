use crate::scalars::{DNum, DVar};
use crate::Ptr;

#[derive(derive_new::new, Clone)]
pub struct DLinTerm<'d> {
    coef: Option<Ptr<'d, DNum<'d>>>,
    var: Ptr<'d, DVar<'d>>,
}
