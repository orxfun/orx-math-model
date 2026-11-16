use crate::{Dim, VarType};

pub struct DV<'d> {
    typ: VarType,
    dim: Dim,
    a: &'d (),
}
