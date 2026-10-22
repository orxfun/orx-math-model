use super::{build_stage::BuildStage, model_bag::ModelBag};
use core::fmt::Debug;

pub struct ModRef<B: BuildStage> {
    ptr: B::ModPtr,
}

impl<B: BuildStage> ModRef<B> {
    fn matches_model(&self, other: &Self) -> bool {
        B::models_match(&self.ptr, &other.ptr)
    }

    fn bag(&self) -> &ModelBag<B> {
        B::bag_ref(&self.ptr)
    }
}

impl<B: BuildStage> Debug for ModRef<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModRef({})", B::stage_str())
    }
}
