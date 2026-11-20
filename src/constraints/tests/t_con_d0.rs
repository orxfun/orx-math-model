use crate::constraints::Constraint;
use crate::{con, eq, geq, leq, sum1, Model, SymWrap, VarType};
use alloc::string::String;
use alloc::{format, vec};

fn exp_str<'m, 'd>(con: Constraint<'m, 'd>) -> String {
    con.data().eval_to_string()
}

#[test]
fn t_con_d0_cint() {
    // data
    let s0_data = vec![2, 3, 0, 1];
    let p1_data = vec![1.0, 0.0, 7.0, 9.0];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let x = m.var1("x", VarType::Binary, s0);
    let p = m.par1_num("p", s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    p.bind(|i| p1_data[i]).unwrap();

    // running

    let x_ = "x_"; // TODO: var-name-fix

    let c = con(3, leq, x[2]);
    assert_eq!(exp_str(c), format!("3 <= {x_}2\n"));

    let c = con(3, eq, 0 * x[2]);
    assert_eq!(exp_str(c), format!("3 = 0\n"));

    let c = con(7 - p[2], leq, x[2]);
    assert_eq!(exp_str(c), format!("0 <= {x_}2\n"));

    let c = con(7 - p[2], geq, x[2] * p[1]);
    assert_eq!(exp_str(c), format!("0 >= 0\n"));

    let c = con(p[2], leq, x[2] * p[0] + 2);
    assert_eq!(exp_str(c), format!("7 <= 1 {x_}2 + 2\n"));

    let c = con(p[2], leq, x[2] * p[0] + sum1(s0, |i| p[i] * x[i]));
    assert_eq!(
        exp_str(c),
        format!("7 <= 1 {x_}2 + 7 {x_}2 + 9 {x_}3 + 1 {x_}0\n")
    );

    let c = con(x[0] * p[2], leq, x[2] * p[0] + sum1(s0, |i| p[i] * x[i]));
    assert_eq!(
        exp_str(c),
        format!("7 {x_}0 <= 1 {x_}2 + 7 {x_}2 + 9 {x_}3 + 1 {x_}0\n")
    );
}

#[test]
fn t_con_d0_ord() {
    // data
    let s0_data = vec![2, 3, 0, 1];

    // modeling
    let m = Model::new();
    let s0 = m.ord_set("s0");
    let x = m.var1("x", VarType::Binary, s0);
    let p = m.par1_num("p", s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    p.bind(|i| 2 * i + 1).unwrap();

    // running

    let x_ = "x_"; // TODO: var-name-fix

    let c = con(p[2], leq, x[2] * p[0] + 2);
    assert_eq!(exp_str(c), format!("5 <= 1 {x_}2 + 2\n"));

    let c = con(p[2], leq, x[2] * p[0] + sum1(s0, |i| p[i] * x[i]));
    assert_eq!(
        exp_str(c),
        format!("5 <= 1 {x_}2 + 5 {x_}2 + 7 {x_}3 + 1 {x_}0 + 3 {x_}1\n")
    );

    let c = con(p[2], leq, x[2] * p[0] + sum1(s0, |i| i * x[i]));
    assert_eq!(
        exp_str(c),
        format!("5 <= 1 {x_}2 + 2 {x_}2 + 3 {x_}3 + 1 {x_}1\n")
    );

    let c = con(p[2], leq, x[2] * p[0] + sum1(s0, |i| i * x[i + 1]));
    assert_eq!(
        exp_str(c),
        format!("5 <= 1 {x_}2 + 2 {x_}3 + 3 {x_}4 + 1 {x_}2\n")
    );
}
