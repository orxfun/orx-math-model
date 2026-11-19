use crate::{forall, sum, Model, SymWrap, VarType};

#[test]
fn t_sum() {
    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let s1 = m.cat_set_str();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let x1 = m.var1(VarType::Binary, s0);
    let x2 = m.var2(VarType::Binary, s0, s1);

    // running

    let sum = sum(forall(i0), x1[i0]);
    assert_eq!(sum.data().eval_to_string(), "1");
}
