use crate::{sum1, Elements, ElementsIter, LinExpr, Model, SetCatInt, SetSym, SymWrap, VarTyp};
use alloc::format;
use alloc::string::String;
use alloc::vec;

fn exp_str<'m, 'd>(e: LinExpr<'m, 'd>) -> String {
    e.data().eval_to_string()
}

#[test]
fn t_lin_expr_str_d1() {
    // data
    let s0_data = vec![0, 2, 3];
    let p1_data = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let i0 = s0.idx();
    let x = m.var1("x", VarTyp::Binary, s0);
    let p = m.par1_num("p", s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    p.bind(|i| p1_data[i]).unwrap();

    // running
    let mut elements: Elements<usize> = vec![].into();
    let iter = ElementsIter::<SetCatInt>::new(i0.ptr(), &mut elements);

    for (i, _) in iter.enumerate() {
        let ei = s0_data[i];
        let ep = p1_data[s0_data[i]];
        let ex = "x"; // TODO: var-name-fix

        let e = LinExpr::from(x[i0]);
        assert_eq!(exp_str(e), format!("{ex}_{ei}"));

        let e = 3 * x[2];
        assert_eq!(exp_str(e), format!("3 {ex}_2"));

        let e = -3.2 * x[i0];
        assert_eq!(exp_str(e), format!("-3.2 {ex}_{ei}"));

        let e = p[2] * x[i0];
        assert_eq!(exp_str(e), format!("7 {ex}_{ei}"));

        let e = -p[i0] * x[i0];
        assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei}"));

        let e = x[i0] + 1;
        assert_eq!(exp_str(e), format!("{ex}_{ei} + 1"));

        let e = -x[i0] + 1;
        assert_eq!(exp_str(e), format!("-{ex}_{ei} + 1"));

        let e = 3 - p[i0] * x[i0];
        assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei} + 3"));

        let e = 3 + (-p[i0]) * x[i0];
        assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei} + 3"));

        let e = 2 + (-p[i0]) * x[i0] + 9;
        assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei} + 11"));

        let e = 2 + (-p[i0]) * x[i0] - 9;
        assert_eq!(exp_str(e), format!("-{ep} {ex}_{ei} - 7"));

        let e = 2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 3;
        assert_eq!(
            exp_str(e),
            format!("2 {ex}_0 - {ep} {ex}_2 + 1.5 {ex}_{ei} + 4")
        );

        let e = 2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 30.5;
        assert_eq!(
            exp_str(e),
            format!("2 {ex}_0 - {ep} {ex}_2 + 1.5 {ex}_{ei} - 23.5")
        );

        let e = 4 * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5;
        assert_eq!(
            exp_str(e),
            format!("8 {ex}_0 - {} {ex}_2 + 1.5 {ex}_{ei} - 2.5", 4.0 * ep)
        );

        let e = (4 * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5) / 2.0;
        assert_eq!(
            exp_str(e),
            format!("4 {ex}_0 - {} {ex}_2 + 0.75 {ex}_{ei} - 1.25", 2.0 * ep)
        );

        let e = (2 * p[1] * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5) / p[1];
        assert_eq!(
            exp_str(e),
            format!("4 {ex}_0 - {} {ex}_2 + 0.75 {ex}_{ei} - 1.25", 2.0 * ep)
        );

        let e = (2 * p[1] * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5) / p[1] + 30;
        assert_eq!(
            exp_str(e),
            format!("4 {ex}_0 - {} {ex}_2 + 0.75 {ex}_{ei} + 28.75", 2.0 * ep)
        );
    }

    let e = 3 + 4 * x[0] + sum1(s0, |i0| x[i0]);
    assert_eq!(exp_str(e), format!("4 x_0 + x_0 + x_2 + x_3 + 3"));

    let e = 3 + 4 * x[0] + sum1(s0, |i0| x[i0]) * 2;
    assert_eq!(exp_str(e), format!("4 x_0 + 2 x_0 + 2 x_2 + 2 x_3 + 3"));

    let e = 3 + 4 * x[0] + sum1(s0, |i0| x[i0] + 4) * 2 - x[1];
    assert_eq!(
        exp_str(e),
        format!("4 x_0 - x_1 + 2 x_0 + 2 x_2 + 2 x_3 + 27")
    );

    let e = 3 + 4 * x[0] + sum1(s0, |i0| x[i0] + 4 - x[2] * 0) * 2 - x[1];
    assert_eq!(
        exp_str(e),
        format!("4 x_0 - x_1 + 2 x_0 + 2 x_2 + 2 x_3 + 27")
    );

    let e = (3 + 4 * x[0] + sum1(s0, |i0| x[i0] + 4 - x[2] * 0) * 2 - x[1]) / 2 + 5;
    assert_eq!(
        exp_str(e),
        format!("2 x_0 - 0.5 x_1 + 1 x_0 + 1 x_2 + 1 x_3 + 18.5")
    );

    let e = (3 + 4 * x[0] + 0 * sum1(s0, |i0| x[i0] + 4 - x[2] * 0) * 2 - x[1]) / 2 + 5;
    assert_eq!(exp_str(e), format!("2 x_0 - 0.5 x_1 + 6.5"));
}

#[test]
fn t_lin_expr_str_d1_zeroes() {
    // data
    let s0_data = vec![0, 2, 3];
    let p1_data = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let i0 = s0.idx();
    let x = m.var1("x", VarTyp::Binary, s0);
    let p = m.par1_num("p", s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    p.bind(|i| p1_data[i]).unwrap();

    // running
    let mut elements: Elements<usize> = vec![].into();
    let iter = ElementsIter::<SetCatInt>::new(i0.ptr(), &mut elements);

    for (i, _) in iter.enumerate() {
        let ei = s0_data[i];
        let ep = p1_data[s0_data[i]];
        let ex = "x"; // TODO: var-name-fix

        let e = 0 * x[2];
        assert_eq!(exp_str(e), format!(""));

        let e = -0.0 * x[i0];
        assert_eq!(exp_str(e), format!(""));

        let e = (p[2] - 7) * x[i0];
        assert_eq!(exp_str(e), format!(""));

        let e = (-p[i0] + p[i0]) * x[i0];
        assert_eq!(exp_str(e), format!(""));

        let e = (p[i0] - ep) * x[i0] + 1;
        assert_eq!(exp_str(e), format!("1"));

        let e = 3 - 0 * x[i0];
        assert_eq!(exp_str(e), format!("3"));

        let e = 3 - 0 * x[i0] - 7.0;
        assert_eq!(exp_str(e), format!("-4"));

        let e = 2 * x[0] + 7 - (p[i0] - ep) * x[2] + 1.5 * x[i0] - 3;
        assert_eq!(exp_str(e), format!("2 {ex}_0 + 1.5 {ex}_{ei} + 4"));

        let e = 0.0 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 3;
        assert_eq!(exp_str(e), format!("-{ep} {ex}_2 + 1.5 {ex}_{ei} + 4"));

        let e = 2 * x[0] + 7 - p[i0] * x[2] + 0 * x[i0] - 3;
        assert_eq!(exp_str(e), format!("2 {ex}_0 - {ep} {ex}_2 + 4"));

        let e = 0 * (2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 30.5);
        assert_eq!(exp_str(e), format!(""));

        let e = 0 * (2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 30.5) + 12.0;
        assert_eq!(exp_str(e), format!("12"));

        let e = -15.5 + 0 * (2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 30.5) + 12.0;
        assert_eq!(exp_str(e), format!("-3.5"));

        let e = (-15.5 + 0 * (2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 30.5) + 12.0) / 2;
        assert_eq!(exp_str(e), format!("-1.75"));
    }
}
