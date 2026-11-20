use crate::vars::Binary;
use crate::{sum1, sum2, Model, SymWrap, VarTyp};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

#[test]
fn t_sum_d1() {
    // data
    let s0_data = vec![0, 1, 7, 3];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let x1 = m.var1::<Binary, _>("x", s0, 0, 1);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running

    let ex1 = "x"; // TODO: var-name-fix

    let e = sum1(s0, |i0| x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("{ex1}_0 + {ex1}_1 + {ex1}_7 + {ex1}_3")
    );

    let e = sum1(s0, |i0| 3 * x1[i0] + 2);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_0 + 3 {ex1}_1 + 3 {ex1}_7 + 3 {ex1}_3 + 8")
    );

    let e = sum1(s0, |i0| 3 * x1[i0] + 2.5 + 7 * x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_0 + 7 {ex1}_0 + 3 {ex1}_1 + 7 {ex1}_1 + 3 {ex1}_7 + 7 {ex1}_7 + 3 {ex1}_3 + 7 {ex1}_3 + 10")
    );

    let e = sum1(s0, |i0| 3 * x1[3] + 2.5 + 7 * x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex1}_3 + 7 {ex1}_0 + 3 {ex1}_3 + 7 {ex1}_1 + 3 {ex1}_3 + 7 {ex1}_7 + 3 {ex1}_3 + 7 {ex1}_3 + 10")
    );
}

#[test]
fn t_sum_d1_zeroes() {
    // data
    let s0_data = vec![0, 1, 7, 3];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let x1 = m.var1::<Binary, _>("x", s0, 0, 1);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running

    let e = sum1(s0, |i0| 0 * x1[i0]);
    assert_eq!(e.data().eval_to_string(), "");

    let e = sum1(s0, |i0| -3 + 0 * x1[i0] + 3);
    assert_eq!(e.data().eval_to_string(), "");
}

#[test]
fn t_sum_d2() {
    // data
    let values = |i0: usize| {
        [7, 9]
            .into_iter()
            .enumerate()
            .map(|(i, v)| ((2 + i).to_string(), (10 * i0 + v) as f64))
            .collect::<HashMap<_, _>>()
    };
    let par_data: Vec<HashMap<String, f64>> = (0..3).map(|i0| values(i0)).collect();
    let s0_data = vec![0, 1, 2];
    let s1_data: Vec<_> = [2, 3].into_iter().map(|x| x.to_string()).collect();

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let s1 = m.cat_set_str("s1");
    let x1 = m.var1::<Binary, _>("x", s0, 0, 1);
    let x2 = m.var2::<Binary, _, _>("y", s0, s1, 0, 1);
    let par = m.par2_num("p", s0, s1);

    // binding
    par.bind(|i, j| par_data[i][j]).unwrap();
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| &s1_data).unwrap();

    // running

    let ex1 = "x"; // TODO: var-name-fix
    let ex2 = "x"; // TODO: var-name-fix

    let e = sum2(s0, s1, |i0, _| x1[i0]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("{ex1}_0 + {ex1}_0 + {ex1}_1 + {ex1}_1 + {ex1}_2 + {ex1}_2")
    );

    let e = sum2(s0, s1, |i0, i1| x2[(i0, i1)]);
    assert_eq!(
        e.data().eval_to_string(),
        format!("{ex2}_0_2 + {ex2}_0_3 + {ex2}_1_2 + {ex2}_1_3 + {ex2}_2_2 + {ex2}_2_3")
    );

    let e = sum2(s0, s1, |i0, i1| 3 + x2[(i0, i1)] + 7);
    assert_eq!(
        e.data().eval_to_string(),
        format!("{ex2}_0_2 + {ex2}_0_3 + {ex2}_1_2 + {ex2}_1_3 + {ex2}_2_2 + {ex2}_2_3 + 60")
    );

    let e = sum2(s0, s1, |i0, i1| 3 * x2[(i0, i1)] + 7);
    assert_eq!(
        e.data().eval_to_string(),
        format!("3 {ex2}_0_2 + 3 {ex2}_0_3 + 3 {ex2}_1_2 + 3 {ex2}_1_3 + 3 {ex2}_2_2 + 3 {ex2}_2_3 + 42")
    );

    let e = sum2(s0, s1, |i0, i1| par[(i0, i1)] * x2[(i0, i1)] + 7);
    assert_eq!(
        e.data().eval_to_string(),
        format!("7 {ex2}_0_2 + 9 {ex2}_0_3 + 17 {ex2}_1_2 + 19 {ex2}_1_3 + 27 {ex2}_2_2 + 29 {ex2}_2_3 + 42")
    );
}

#[test]
fn t_sum_d2_zeroes() {
    // data
    let values = |i0: usize| {
        [7, 9]
            .into_iter()
            .enumerate()
            .map(|(i, v)| ((2 + i).to_string(), (10 * i0 + v) as f64))
            .collect::<HashMap<_, _>>()
    };
    let par_data: Vec<HashMap<String, f64>> = (0..3).map(|i0| values(i0)).collect();
    let s0_data = vec![0, 1, 2];
    let s1_data: Vec<_> = [2, 3].into_iter().map(|x| x.to_string()).collect();

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let s1 = m.cat_set_str("s1");
    let x1 = m.var1::<Binary, _>("x", s0, 0, 1);
    let x2 = m.var2::<Binary, _, _>("y", s0, s1, 0, 1);
    let par = m.par2_num("p", s0, s1);

    // binding
    par.bind(|i, j| par_data[i][j]).unwrap();
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| &s1_data).unwrap();

    // running

    let ex2 = "x"; // TODO: var-name-fix

    let e = sum2(s0, s1, |i0, _i1| 0 * x1[i0]);
    assert_eq!(e.data().eval_to_string(), "");

    let e = sum2(s0, s1, |i0, i1| 7 + 0 * x2[(i0, i1)] - 7.0);
    assert_eq!(e.data().eval_to_string(), "");

    let e = sum2(s0, s1, |i0, i1| {
        par[(i0, i1)] * x2[(i0, i1)] + 7 + 0 * x2[(i0, i1)]
    });
    assert_eq!(
        e.data().eval_to_string(),
        format!("7 {ex2}_0_2 + 9 {ex2}_0_3 + 17 {ex2}_1_2 + 19 {ex2}_1_3 + 27 {ex2}_2_2 + 29 {ex2}_2_3 + 42")
    );

    let e = sum2(s0, s1, |i0, i1| {
        (par[(i0, i1)] - par[(i0, i1)]) * x2[(i0, i1)] + 7 + par[(i0, i1)] * 0 * x2[(i0, i1)]
    });
    assert_eq!(e.data().eval_to_string(), "42");
}
