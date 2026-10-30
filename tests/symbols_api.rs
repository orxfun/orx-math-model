use orx_math_model::*;

#[test]
fn sets() {
    let m = Model::new();

    let si = m.cat_set_int();
    let ss = m.cat_set_str();
    let i = m.ord_set();
}

#[test]
fn idx_scalars() {
    let m = Model::new();

    let si = m.cat_set_int();
    let ss = m.cat_set_str();
    let i = m.ord_set();

    let si_i = si.idx();
    let ss_i = ss.idx();
    let i_i = i.idx();
}
