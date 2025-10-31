use crate::symbols::sets::set_gen::SetGen;

pub trait IntoSet {
    type SetGen<'m>: SetGen<'m>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m>;
}
