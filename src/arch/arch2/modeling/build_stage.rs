use super::model_bag::ModelBag;
use std::{fmt::Debug, rc::Rc};

pub trait BuildStage: Clone + Debug {
    type ModPtr;

    fn models_match(a: &Self::ModPtr, b: &Self::ModPtr) -> bool;

    fn bag_ref(ptr: &Self::ModPtr) -> &ModelBag<Self>;

    fn stage_str() -> &'static str;
}

#[derive(Clone, Copy, Debug)]
pub struct Building;

impl BuildStage for Building {
    type ModPtr = *const ModelBag<Self>;

    fn models_match(a: &Self::ModPtr, b: &Self::ModPtr) -> bool {
        *a as usize == *b as usize
    }

    fn bag_ref(ptr: &Self::ModPtr) -> &ModelBag<Self> {
        unsafe { &*(*ptr) as &ModelBag<Self> }
    }

    fn stage_str() -> &'static str {
        "Building"
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Built;

impl BuildStage for Built {
    type ModPtr = Rc<ModelBag<Self>>;

    fn models_match(a: &Self::ModPtr, b: &Self::ModPtr) -> bool {
        Rc::ptr_eq(a, b)
    }

    fn bag_ref(ptr: &Self::ModPtr) -> &ModelBag<Self> {
        ptr
    }

    fn stage_str() -> &'static str {
        "Built"
    }
}
