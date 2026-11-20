use crate::constraints::Constraint;
use crate::sets::CatInt;
use crate::{eq, for1, for2, geq, leq, sum1, sum2, Model, SetCatInt, SetSym, SymWrap, VarType};
use alloc::string::String;
use alloc::{format, vec};

fn exp_str<'m, 'd>(con: Constraint<'m, 'd>) -> String {
    con.data().eval_to_string()
}

#[test]
fn t_con_d1() {
    // data
    let s0_data = vec![2, 3, 0, 1];
    let p1_data = vec![1.0, 0.0, 7.0, 9.0];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let x = m.var1(VarType::Binary, s0);
    let p = m.par1_num(s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    p.bind(|i| p1_data[i]).unwrap();

    // running

    let x_ = "x_"; // TODO: var-name-fix

    let c = for1(s0, |i| (x[i], leq, 1.0));
    assert_eq!(
        exp_str(c),
        format!(
            r#"{x_}2 <= 1
{x_}3 <= 1
{x_}0 <= 1
{x_}1 <= 1
"#
        )
    );

    let c = for1(s0, |i| (x[i], leq, sum1(s0, |i| x[i] + 1)));
    assert_eq!(
        exp_str(c),
        format!(
            r#"{x_}2 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
{x_}3 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
{x_}0 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
{x_}1 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
"#
        )
    );

    let c = for1(s0, |i| (x[i], leq, sum1(s0, |j| p[i] * x[j] + 1)));
    assert_eq!(
        exp_str(c),
        format!(
            r#"{x_}2 <= 7 {x_}2 + 7 {x_}3 + 7 {x_}0 + 7 {x_}1 + 4
{x_}3 <= 9 {x_}2 + 9 {x_}3 + 9 {x_}0 + 9 {x_}1 + 4
{x_}0 <= 1 {x_}2 + 1 {x_}3 + 1 {x_}0 + 1 {x_}1 + 4
{x_}1 <= 4
"#
        )
    );
}
