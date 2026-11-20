use crate::{BoundsOld, Model, VarType};

#[test]
fn v0_int_cint() {
    // modeling
    let m = Model::new();
    let _var = m.var0("x", VarType::Binary);
}
