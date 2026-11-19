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
    let v1 = m.var1(VarType::Binary, s0);
    let p1 = m.par1_num(s0);

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    p1.bind(|i| p1_data[i]).unwrap();

    // running
    let mut elements: Elements<usize> = vec![].into();
    let iter = ElementsIter::<SetCatInt>::new(i0, &mut elements);

    for (i, _) in iter.enumerate() {
        let exp_i0 = s0_data[i];
        let exp_p1 = p1_data[s0_data[i]];

        let e = LinExpr::from(v1[i0]);
        assert_eq!(exp_str(e), format!("x_{exp_i0}"));

        let e = 3 * v1[2];
        assert_eq!(exp_str(e), format!("3 x_2"));

        let e = -3.2 * v1[i0];
        assert_eq!(exp_str(e), format!("-3.2 x_{exp_i0}"));

        let e = p1[2] * v1[i0];
        assert_eq!(exp_str(e), format!("7 x_{exp_i0}"));

        // let e = -p1[2] * v1[i0];
        // assert_eq!(exp_str(e), format!("7 x_{exp_i0}"));
    }
}
