use crate::{set, Model};

#[test]
fn t_set_fun() {
    let m = Model::new();

    let org = set(&m, "org").cat().str();
    let dst = set(&m, "dst").cat().str();
    let t = set(&m, "t").ord();
    let idx = set(&m, "i").cat().int();
}
