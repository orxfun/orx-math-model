use crate::modeling::model_data::ModelData;
use std::{fmt::Debug, rc::Rc};

pub trait ModelStages: Clone + Debug {
    fn core(&self) -> &ModelData<Self>;
    fn is_same_model(&self, other: &Self) -> bool;
}

#[derive(Clone, Copy)]
pub struct Building(pub(super) *const ModelData<Building>);

#[derive(Clone)]
pub struct Built(pub(super) Rc<ModelData<Built>>);

impl ModelStages for Building {
    fn core(&self) -> &ModelData<Self> {
        unsafe { &*(self.0) as &ModelData<Building> }
    }
    fn is_same_model(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ModelStages for Built {
    fn core(&self) -> &ModelData<Self> {
        &self.0
    }
    fn is_same_model(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Debug for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Building").finish()
    }
}
impl Debug for Built {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Built").finish()
    }
}
