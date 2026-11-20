use crate::{AsF64, Model, SetCatInt, SetCatStr, SetOrd, SymWrap};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use std::collections::HashMap;

#[test]
fn t_p0() {
    // data
    let val = Box::new(27.0);

    // modeling
    let m = Model::new();
    let pi = m.par0_int();
    let pn = m.par0_num();

    // binding
    pi.bind(|| *val).unwrap();
    pn.bind(|| 42).unwrap();
}
