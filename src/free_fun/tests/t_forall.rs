use crate::{over, Model, SetSym, SymWrap};

#[test]
fn t_forall() {
    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let s1 = m.cat_set_str();
    let i0 = s0.idx();
    let i1 = s1.idx();

    let idxs1 = over(i0);
    let idxs2 = over((i0, i1));

    // running

    let idxs = idxs1.0.to_idx_vec();
    assert_eq!(idxs.len(), 1);
    assert!(idxs[0].ptr_eq(i0.ptr()));

    let idxs = idxs2.0.to_idx_vec();
    assert_eq!(idxs.len(), 2);
    assert!(idxs[0].ptr_eq(i0.ptr()));
    assert!(idxs[1].ptr_eq(i1.ptr()));
}
