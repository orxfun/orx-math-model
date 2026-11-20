use crate::{par, set, Model, SetCatInt};

#[test]
fn t_model_api() {
    // modeling

    let m = Model::new();

    let org = set(&m, "O").cat().str(); // TODO: unite this call
    let dst = set(&m, "D").cat().str();
    let per = set(&m, "P").ord();

    let num_per = par("T").d0(&m);
    let cost = par("c").d2(org, dst);
}
