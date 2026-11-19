use crate::{
    Bounds, Elements, ElementsIter, LinExpr, Model, Set, SetCatInt, SetCatStr, SetOrd, SymWrap,
    VarType,
};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

fn exp_str<'m, 'd>(e: LinExpr<'m, 'd>) -> String {
    e.data().eval_to_string()
}

#[test]
fn t_lin_expr_str_d2() {
    type I0 = SetCatStr;
    type I1 = SetOrd;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let values = vec![vec![0u32, 1], vec![2, 3], vec![4, 5]];
    let par_data: HashMap<String, Vec<u32>> = values
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();
    let s0_data: Vec<_> = vec![0, 1, 2].into_iter().map(|x| x.to_string()).collect();
    let s1_data = vec![0i64, 1];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_str();
    let s1 = m.ord_set();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let p = m.par2_int(s0, s1);
    let x = m.var2(VarType::Cont(Bounds::default()), s0, s1);

    // binding
    p.bind(|i, j| par_data[i][j as usize]).unwrap();
    s0.bind(|| &s0_data).unwrap();
    s1.bind(|| s1_data.iter().copied()).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0, &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1, &mut el1);
        for (j, _) in it1.enumerate() {
            let ei = &s0_data[i];
            let ej = s1_data[j] as usize;
            let ep = par_data[ei][ej];
            let ex = "x"; // TODO: var-name-fix

            // let e = LinExpr::from(x[(i0, i1)]);
            // assert_eq!(exp_str(e), format!("{ex}_{ei}_{ej}"));

            // let e = 3 * x[("bonn", 2)];
            // assert_eq!(exp_str(e), format!("3 {ex}_bonn_2"));

            // let e = -3.2 * x[(i0, i1)];
            // assert_eq!(exp_str(e), format!("-3.2 {ex}_{ei}_{ej}"));

            // let e = p[("2", 0)] * x[(i0, i1)];
            // assert_eq!(exp_str(e), format!("4 {ex}_{ei}_{ej}"));

            // let e = -p[(i0, i1)] * x[(i0, i1)];
            // assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei}_{ej}"));

            // let e = x[(i0, i1)] + 1;
            // assert_eq!(exp_str(e), format!("{ex}_{ei}_{ej} + 1"));

            // let e = -x[(i0, i1)] + 1;
            // assert_eq!(exp_str(e), format!("-{ex}_{ei}_{ej} + 1"));

            // let e = 3 - p[(i0, i1)] * x[(i0, i1)];
            // assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei}_{ej} + 3"));

            // let e = 3 + (-p[(i0, i1)]) * x[(i0, i1)];
            // assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei}_{ej} + 3"));

            // let e = 2 + (-p[(i0, i1)]) * x[(i0, i1)] + 9;
            // assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei}_{ej} + 11"));

            // let e = 2 + (-p[(i0, i1)]) * x[(i0, i1)] - 9;
            // assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei}_{ej} - 7"));

            let e = 2 * x[("mia", 1)] + 7 - p[(i0, i1)] * x[("sin", 3)] + 1.5 * x[(i0, i1)] - 3;
            assert_eq!(
                exp_str(e),
                format!("2 {ex}_mia_1 - {ep} {ex}_sin_3 + 1.5 {ex}_{ei}_{ej} + 4")
            );

            break;
        }
    }
}
