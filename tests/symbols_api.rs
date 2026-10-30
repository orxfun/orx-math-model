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

    let ii = si.idx();
    let is = ss.idx();
}
