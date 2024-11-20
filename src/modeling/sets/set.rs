use crate::modeling::{build_stage::BuildStage, sym_ref::SymRef};

pub struct Set<B: BuildStage>(SymRef<B>);
