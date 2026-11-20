use std::collections::HashMap;

use crate::constraints::Constraint;
use crate::sets::CatInt;
use crate::{
    con, eq, for1, for2, geq, leq, sum1, sum2, Bounds, Model, SetCatInt, SetSym, SymWrap, VarType,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

fn exp_str<'m, 'd>(con: Constraint<'m, 'd>) -> String {
    con.data().eval_to_string()
}

#[test]
fn t_con_d2() {
    // data
    fn arr_to_loc<const N: usize>(arr: [&str; N]) -> Vec<String> {
        arr.into_iter().map(|x| x.to_string()).collect()
    }

    let d_org = arr_to_loc(["SIN", "AMS", "MIA"]);
    let d_dst = arr_to_loc(["BRU", "FRA"]);
    let d_cost: HashMap<String, HashMap<String, f64>> = [
        ("SIN", vec![("BRU", 10.0), ("FRA", 20.0)]),
        ("AMS", vec![("FRA", 12.0)]),
        ("MIA", vec![("BRU", 25.0)]),
    ]
    .into_iter()
    .map(|(o, x)| {
        (
            o.to_string(),
            HashMap::from_iter(x.into_iter().map(|(d, c)| (d.to_string(), c))),
        )
    })
    .collect();

    // modeling
    let m = Model::new();
    let org = m.cat_set_str();
    let dst = m.cat_set_str();
    let c = m.par2(org, dst);
    let y = m.var1(VarType::Binary, org);
    let x = m.var2(VarType::Cont(Bounds::default()), org, dst);

    // binding
    org.bind(|| &d_org).unwrap();
    dst.bind(|| &d_dst).unwrap();
    c.bind(|o, d| {
        d_cost
            .get(o)
            .and_then(|x| x.get(d).copied())
            .unwrap_or(1000.0)
    })
    .unwrap();

    // running

    let x_ = "x_"; // TODO: var-name-fix
    let y_ = "y_"; // TODO: var-name-fix

    //     let c = for1(s0, |i| con(x[i], leq, 1.0));
    //     assert_eq!(
    //         exp_str(c),
    //         format!(
    //             r#"{x_}2 <= 1
    // {x_}3 <= 1
    // {x_}0 <= 1
    // {x_}1 <= 1
    // "#
    //         )
    //     );

    //     let c = for1(s0, |i| con(x[i], leq, sum1(s0, |i| x[i] + 1)));
    //     assert_eq!(
    //         exp_str(c),
    //         format!(
    //             r#"{x_}2 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
    // {x_}3 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
    // {x_}0 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
    // {x_}1 <= {x_}2 + {x_}3 + {x_}0 + {x_}1 + 4
    // "#
    //         )
    //     );

    //     let c = for1(s0, |i| con(x[i], leq, sum1(s0, |j| p[i] * x[j] + 1)));
    //     assert_eq!(
    //         exp_str(c),
    //         format!(
    //             r#"{x_}2 <= 7 {x_}2 + 7 {x_}3 + 7 {x_}0 + 7 {x_}1 + 4
    // {x_}3 <= 9 {x_}2 + 9 {x_}3 + 9 {x_}0 + 9 {x_}1 + 4
    // {x_}0 <= 1 {x_}2 + 1 {x_}3 + 1 {x_}0 + 1 {x_}1 + 4
    // {x_}1 <= 4
    // "#
    //         )
    //     );
}
