use crate::{
    modeling::stages::{Building, Built},
    modeling::{model_data::ModelData, reference::SymRef},
};
use smallvec::SmallVec;
use std::{ops::Deref, rc::Rc};

pub(crate) const EXPECTED_LEN: usize = 8;

#[derive(Debug, Clone)]
pub(crate) struct Smallvec<T>(SmallVec<[T; EXPECTED_LEN]>);

impl<T> Smallvec<T> {
    pub(crate) fn empty() -> Self {
        Self(SmallVec::default())
    }
    pub(crate) fn singleton(value: T) -> Self {
        let mut vec = Self::empty();
        vec.push(value);
        vec
    }
    pub(crate) fn from_iter<I>(iter: I, capacity: Option<usize>) -> Self
    where
        I: Iterator<Item = T>,
    {
        let mut vec = if let Some(cap) = capacity {
            SmallVec::with_capacity(cap)
        } else {
            SmallVec::new()
        };
        for val in iter {
            vec.push(val)
        }
        Self(vec)
    }

    pub(crate) fn push(&mut self, value: T) {
        self.0.push(value)
    }
}

impl<T> Deref for Smallvec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const D: usize> From<[T; D]> for Smallvec<T> {
    fn from(value: [T; D]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl Smallvec<SymRef<Building>> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> Smallvec<SymRef<Built>> {
        let iter = self.iter().map(|symref| symref.build(model.clone()));
        Smallvec::from_iter(iter, Some(self.len()))
    }
}
