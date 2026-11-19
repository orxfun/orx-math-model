use crate::sets::CatInt;
use crate::{con, eq, for1, geq, leq, Model, SetCatInt, SetSym, VarType};
use alloc::vec;

#[test]
fn con_d0() {
    // data
    let s0_data = vec![0, 7, 3];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let i0 = s0.idx();
    let x = m.var1(VarType::Binary, s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running

    let ex1 = "x"; // TODO: var-name-fix

    // let c = con_depr(3, leq, x[2], forall(()));

    let c = con(3, leq, x[2]);

    let c = for1(s0, |i| con(3, leq, x[2]));

    let c = for1(s0, |i| con(3, leq, x[2]));
}
