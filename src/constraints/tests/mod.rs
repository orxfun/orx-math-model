use crate::{con, constraints::IneqTyp, forall, Model, VarType};
use alloc::vec;

#[test]
fn con_d0() {
    // data
    let s0_data = vec![0, 1, 7, 3];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let i0 = s0.idx();
    let x = m.var1(VarType::Binary, s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running

    let ex1 = "x"; // TODO: var-name-fix

    let c = con(3, IneqTyp::Eq, 4, forall(()));
}
