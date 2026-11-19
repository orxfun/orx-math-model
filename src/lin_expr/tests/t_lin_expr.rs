use crate::{
    Bounds, Elements, ElementsIter, LinExpr, Model, Set, SetCatInt, SetCatStr, SetOrd, SymWrap,
    VarType,
};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

fn exp_str<'m, 'd>(e: LinExpr<'m, 'd>) -> String {
    e.data().eval_to_string()
}

#[test]
fn t_lin_expr_d1() {
    // data
    let s0_data = vec![0, 2, 3];
    let p1_data = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let i0 = s0.idx();
    let x = m.var1(VarType::Binary, s0);
    let p = m.par1_num(s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    p.bind(|i| p1_data[i]).unwrap();

    // running
    let mut elements: Elements<usize> = vec![].into();
    let iter = ElementsIter::<SetCatInt>::new(i0, &mut elements);

    for (i, _) in iter.enumerate() {
        let ei = s0_data[i];
        let ep = p1_data[s0_data[i]];

        let e = LinExpr::from(x[i0]);
        assert_eq!(exp_str(e), format!("x_{ei}"));

        let e = 3 * x[2];
        assert_eq!(exp_str(e), format!("3 x_2"));

        let e = -3.2 * x[i0];
        assert_eq!(exp_str(e), format!("-3.2 x_{ei}"));

        let e = p[2] * x[i0];
        assert_eq!(exp_str(e), format!("7 x_{ei}"));

        let e = -p[i0] * x[i0];
        assert_eq!(exp_str(e), format!("-{ep} x_{ei}"));

        let e = x[i0] + 1;
        assert_eq!(exp_str(e), format!("x_{ei} + 1"));

        let e = -x[i0] + 1;
        assert_eq!(exp_str(e), format!("-x_{ei} + 1"));

        let e = 3 - p[i0] * x[i0];
        assert_eq!(exp_str(e), format!("-{ep} x_{ei} + 3"));

        let e = 3 + (-p[i0]) * x[i0];
        assert_eq!(exp_str(e), format!("-{ep} x_{ei} + 3"));

        let e = 2 + (-p[i0]) * x[i0] + 9;
        assert_eq!(exp_str(e), format!("-{ep} x_{ei} + 11"));

        let e = 2 + (-p[i0]) * x[i0] - 9;
        assert_eq!(exp_str(e), format!("-{ep} x_{ei} - 7"));

        let e = 2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 3;
        assert_eq!(exp_str(e), format!("2 x_0 - {ep} x_2 + 1.5 x_{ei} + 4"));

        let e = 2 * x[0] + 7 - p[i0] * x[2] + 1.5 * x[i0] - 30.5;
        assert_eq!(exp_str(e), format!("2 x_0 - {ep} x_2 + 1.5 x_{ei} - 23.5"));

        let e = 4 * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5;
        assert_eq!(
            exp_str(e),
            format!("8 x_0 - {} x_2 + 1.5 x_{ei} - 2.5", 4.0 * ep)
        );

        let e = (4 * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5) / 2.0;
        assert_eq!(
            exp_str(e),
            format!("4 x_0 - {} x_2 + 0.75 x_{ei} - 1.25", 2.0 * ep)
        );

        let e = (2 * p[1] * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5) / p[1];
        assert_eq!(
            exp_str(e),
            format!("4 x_0 - {} x_2 + 0.75 x_{ei} - 1.25", 2.0 * ep)
        );

        let e = (2 * p[1] * (2 * x[0] + 7 - p[i0] * x[2]) + 1.5 * x[i0] - 30.5) / p[1] + 30;
        assert_eq!(
            exp_str(e),
            format!("4 x_0 - {} x_2 + 0.75 x_{ei} + 28.75", 2.0 * ep)
        );
    }
}
