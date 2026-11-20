use crate::{par, set, var, Model, SetCatInt};

#[test]
fn t_model_api() {
    // modeling

    let m = Model::new();

    let org = set(&m, "O").cat().str(); // TODO: unite this call
    let dst = set(&m, "D").cat().str();
    let per = set(&m, "P").ord();

    let horizon = par("T").d0(&m);
    let cost = par("c").d2(org, dst);

    let y = var("y").binary().d1(org);
    let x = var("x").d2(org, dst).nonneg();
    let t = var("t").d0(&m).bounded(1, horizon);
}
