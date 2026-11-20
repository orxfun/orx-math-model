use crate::{Model, SymWrap, VarType};

#[test]
fn t_v0() {
    // modeling
    let m = Model::new();
    let var = m.var0("x", VarType::Binary);

    unsafe { var.data_mut() }.set_bound_d0(true, 10);
    unsafe { var.data_mut() }.set_bound_d0(true, 12.0);
}
