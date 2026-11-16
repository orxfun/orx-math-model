use crate::{lin_term::LinTerm, Var};

impl<'m, 'd> From<Var<'m, 'd>> for LinTerm<'m, 'd> {
    fn from(var: Var<'m, 'd>) -> Self {
        // Self::new(coef, var)
        todo!()
    }
}
