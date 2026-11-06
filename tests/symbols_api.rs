use orx_math_model::*;

#[test]
fn sets() {
    let m = Model::new();

    let si = m.set_cat_int();
    let ss = m.set_cat_str();
    let i = m.set_ord();
}

#[test]
fn idx_scalars() {
    let m = Model::new();

    let si = m.set_cat_int();
    let ss = m.set_cat_str();
    let i = m.set_ord();

    let si_i = si.idx();
    let ss_i = ss.idx();
    let i_i = i.idx();
}
