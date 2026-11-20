use crate::{set, CatInt, CatStr, Model, Ord};

#[test]
fn t_set_fun() {
    let m = Model::new();

    let _org: CatStr<'_, '_> = set(&m, "org").cat().str();
    let _dst: CatStr<'_, '_> = set(&m, "dst").cat().str();
    let _t: Ord<'_, '_> = set(&m, "t").ord();
    let _idx: CatInt<'_, '_> = set(&m, "i").cat().int();
}
