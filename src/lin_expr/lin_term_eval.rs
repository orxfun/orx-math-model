use crate::vars::DVars;
use crate::Ptr;

#[derive(derive_new::new, Clone)]
pub struct LinTermEval<'d> {
    pub coef: f64,
    pub parent: Ptr<'d, DVars<'d>>,
}
