use crate::modeling::build_stage::BuildStage;
use core::marker::PhantomData;

pub struct ModelBag<B: BuildStage> {
    phantom: PhantomData<B>,
}
