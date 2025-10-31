use crate::symbols::sets::set_gen::SetGen;

pub trait IntoSet {
    type SetGen: SetGen + 'static;

    fn into_set(self) -> Self::SetGen;
}
