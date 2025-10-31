use crate::symbols::sets::set_gen::SetGen;

pub trait IntoSet {
    type SetGen: SetGen;

    fn into_set_gen<'m>(self) -> Self::SetGen;
}
