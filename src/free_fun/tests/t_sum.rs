use crate::{forall, sum, Model, SymWrap, VarType};
use alloc::format;
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn t_sum_d1() {
    // data
    let s0_data = vec![0, 1, 7, 3];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let i0 = s0.idx();
    let x1 = m.var1(VarType::Binary, s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running

    let ex1 = "x"; // TODO: var-name-fix

    let e = sum(forall(i0), x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("{ex1}_0 + {ex1}_1 + {ex1}_7 + {ex1}_3")
    );

    let e = sum(forall(i0), 3 * x1[i0] + 2);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_0 + 3 {ex1}_1 + 3 {ex1}_7 + 3 {ex1}_3 + 8")
    );

    let e = sum(forall(i0), 3 * x1[i0] + 2.5 + 7 * x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_0 + 7 {ex1}_0 + 3 {ex1}_1 + 7 {ex1}_1 + 3 {ex1}_7 + 7 {ex1}_7 + 3 {ex1}_3 + 7 {ex1}_3 + 10")
    );

    let e = sum(forall(i0), 3 * x1[3] + 2.5 + 7 * x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_3 + 7 {ex1}_0 + 3 {ex1}_3 + 7 {ex1}_1 + 3 {ex1}_3 + 7 {ex1}_7 + 3 {ex1}_3 + 7 {ex1}_3 + 10")
    );
}

#[test]
fn t_sum_d2() {
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

    let ex1 = "x"; // TODO: var-name-fix
    let ex2 = "x"; // TODO: var-name-fix

    let e = sum(forall((i0, i1)), x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("{ex1}_0 + {ex1}_1 + {ex1}_7 + {ex1}_3")
    );

    let e = sum(forall(i0), 3 * x1[i0] + 2);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_0 + 3 {ex1}_1 + 3 {ex1}_7 + 3 {ex1}_3 + 8")
    );

    let e = sum(forall(i0), 3 * x1[i0] + 2.5 + 7 * x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_0 + 7 {ex1}_0 + 3 {ex1}_1 + 7 {ex1}_1 + 3 {ex1}_7 + 7 {ex1}_7 + 3 {ex1}_3 + 7 {ex1}_3 + 10")
    );

    let e = sum(forall(i0), 3 * x1[3] + 2.5 + 7 * x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_3 + 7 {ex1}_0 + 3 {ex1}_3 + 7 {ex1}_1 + 3 {ex1}_3 + 7 {ex1}_7 + 3 {ex1}_3 + 7 {ex1}_3 + 10")
    );

    let e = sum(forall((i0, i1)), 3 * x1[3] + 2.5 + 7 * x2[(i0, i1)]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_3 + 7 {ex1}_0 + 3 {ex1}_3 + 7 {ex1}_1 + 3 {ex1}_3 + 7 {ex1}_7 + 3 {ex1}_3 + 7 {ex1}_3 + 10")
    );
}
