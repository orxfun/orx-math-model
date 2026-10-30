use orx_math_model::*;

#[test]
fn sets() {
    let m = Model::new();

    let si = m.set_int();
    let ss = m.set_str();
    let i = m.indices();
}

#[test]
fn idx_scalars() {
    let m = Model::new();

    let si = m.set_int();
    let ss = m.set_str();
    let i = m.indices();

    let si_i = si.idx();
    let ss_i = ss.idx();
    let i_i = i.idx();
}
