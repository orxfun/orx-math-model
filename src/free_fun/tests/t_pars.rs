use crate::{
    par, par_int, set, CatInt, CatStr, IntParD0, IntParD1, IntParD2, Model, NumParD0, NumParD1,
    NumParD2, Ord, SetCatInt, SetCatStr, SetOrd,
};

#[test]
fn t_set_fun() {
    let m = Model::new();

    let org: CatStr<'_, '_> = set(&m, "org").cat().str();
    let dst: CatStr<'_, '_> = set(&m, "dst").cat().str();
    let t: Ord<'_, '_> = set(&m, "t").ord();
    let idx: CatInt<'_, '_> = set(&m, "i").cat().int();

    let _: IntParD0<'_, '_> = par_int("p").d0(&m);
    let _: NumParD0<'_, '_> = par("p").d0(&m);

    let _: IntParD1<'_, '_, SetCatStr> = par_int("p").d1(org);
    let _: NumParD1<'_, '_, SetOrd> = par("p").d1(t);

    let _: IntParD2<'_, '_, SetCatStr, SetCatInt> = par_int("p").d2(org, idx);
    let _: NumParD2<'_, '_, SetOrd, SetCatStr> = par("p").d2(t, dst);
}
