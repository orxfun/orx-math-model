use crate::constraints::Constraint;
use crate::{con, con1, con2, geq, leq, sum1, sum2,  Model, SymWrap, VarTyp};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use std::collections::HashMap;

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
    let org = m.cat_set_str("org");
    let dst = m.cat_set_str("dst");
    let c = m.par2_num("c", org, dst);
    let y = m.var1("y", VarTyp::Binary, org);
    let x = m.var2("x", VarTyp::Continuous, org, dst);

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
    let y_ = "x_"; // TODO: var-name-fix

    let c = con2(org, dst, |o, d| (x[(o, d)], leq, y[o]));
    assert_eq!(
        exp_str(c),
        format!(
            r#"{x_}SIN_BRU <= {y_}SIN
{x_}SIN_FRA <= {y_}SIN
{x_}AMS_BRU <= {y_}AMS
{x_}AMS_FRA <= {y_}AMS
{x_}MIA_BRU <= {y_}MIA
{x_}MIA_FRA <= {y_}MIA
"#
        )
    );

    let c = con2(org, dst, |o, d| (x[(o, d)], geq, sum1(org, |o| y[o])));
    assert_eq!(
        exp_str(c),
        format!(
            r#"{x_}SIN_BRU >= {y_}SIN + {y_}AMS + {y_}MIA
{x_}SIN_FRA >= {y_}SIN + {y_}AMS + {y_}MIA
{x_}AMS_BRU >= {y_}SIN + {y_}AMS + {y_}MIA
{x_}AMS_FRA >= {y_}SIN + {y_}AMS + {y_}MIA
{x_}MIA_BRU >= {y_}SIN + {y_}AMS + {y_}MIA
{x_}MIA_FRA >= {y_}SIN + {y_}AMS + {y_}MIA
"#
        )
    );

    let c = con1(org, |o| (sum1(dst, |d| x[(o, d)]), leq, y[o]));
    assert_eq!(
        exp_str(c),
        format!(
            r#"{x_}SIN_BRU + {x_}SIN_FRA <= {y_}SIN
{x_}AMS_BRU + {x_}AMS_FRA <= {y_}AMS
{x_}MIA_BRU + {x_}MIA_FRA <= {y_}MIA
"#
        )
    );
}
