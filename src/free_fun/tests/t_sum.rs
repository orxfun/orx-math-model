use alloc::string::ToString;
use alloc::{vec, vec::Vec};

use crate::{forall, sum, Model, SymWrap, VarType};

#[test]
fn t_sum() {
    // data
    let s0_data = vec![0, 1, 7, 3];
    let s1_data: Vec<_> = [0, 1, 2, 3].into_iter().map(|x| x.to_string()).collect();

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let s1 = m.cat_set_str();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let x1 = m.var1(VarType::Binary, s0);
    let x2 = m.var2(VarType::Binary, s0, s1);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| &s1_data).unwrap();

    // running

    let e = sum(forall(i0), x1[i0]);
    assert_eq!(e.data().eval_to_string(), "x_0 + x_1 + x_7 + x_3");

    let e = sum(forall(i0), 3 * x1[i0] + 2);
    // assert_eq!(e.data().eval_to_string(), "3 x_0 + 3 x_1 + 3 x_7 + 3 x_3");
}
