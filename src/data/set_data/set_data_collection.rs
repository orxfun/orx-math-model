use crate::symbols::pars::ParCore;
use crate::{symbols::sets::SetCore, Set};
use crate::{Model, Par};

pub trait SetDataCollection<'m> {
    fn into_iter(self) -> impl Iterator<Item = SetCore<'m>>;
}
