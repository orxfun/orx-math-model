use super::{build_stage::BuildStage, mod_ref::ModRef};

pub struct SymRef<B: BuildStage> {
    mod_ref: ModRef<B>,
    idx: usize,
}
