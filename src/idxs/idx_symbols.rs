use crate::idxs::DIdx;
use crate::{SymWrap, Symbol};

// cat int

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxCatInt<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> SymWrap<'m, 'd> for IdxCatInt<'m, 'd> {
    type Data = DIdx<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

// cat str

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxCatStr<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> SymWrap<'m, 'd> for IdxCatStr<'m, 'd> {
    type Data = DIdx<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

// ord

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxOrd<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> SymWrap<'m, 'd> for IdxOrd<'m, 'd> {
    type Data = DIdx<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}
