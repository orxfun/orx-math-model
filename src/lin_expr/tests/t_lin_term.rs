use crate::lin_expr::{Coef, DLinTerm};
use crate::scalars::{DNum, ValNum};
use crate::{Model, SymWrap, VarType};

#[test]
fn lin_term() {
    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int();
    let i0 = s0.idx();
    let s1 = m.cat_set_str();
    let i1 = s1.idx();
    let var1 = m.var1(VarType::Binary, s0);
    let var2 = m.var2(VarType::Binary, s0, s1);
    let v1 = var1[i0];
    let v2 = var2[(i0, i1)];

    // construction

    let term = DLinTerm::new(Coef::One, v1.ptr());
    assert_eq!(term.eval_coef(), 1.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::MinusOne, v1.ptr());
    assert_eq!(term.eval_coef(), -1.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::Con(7.0), v2.ptr());
    assert_eq!(term.eval_coef(), 7.0);
    assert!(term.var().ptr_eq(v2.ptr()));

    let term = DLinTerm::new(
        Coef::Ptr(m.val::<ValNum>(DNum::new_num(|| 42f32)).ptr()),
        v2.ptr(),
    );
    assert_eq!(term.eval_coef(), 42.0);
    assert!(term.var().ptr_eq(v2.ptr()));

    // ops - neg

    let term = DLinTerm::new(Coef::MinusOne, v1.ptr());
    assert_eq!(term.eval_coef(), -1.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::One, v1.ptr()).neg(&m);
    assert_eq!(term.eval_coef(), 1.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::Con(7.0), v2.ptr()).neg(&m);
    assert_eq!(term.eval_coef(), -7.0);
    assert!(term.var().ptr_eq(v2.ptr()));

    let term = DLinTerm::new(
        Coef::Ptr(m.val::<ValNum>(DNum::new_num(|| 42f32)).ptr()),
        v2.ptr(),
    )
    .neg(&m);
    assert_eq!(term.eval_coef(), -42.0);
    assert!(term.var().ptr_eq(v2.ptr()));

    // ops - mul num

    let num = m.val::<ValNum>(DNum::new_num(|| 3)).ptr();

    let term = DLinTerm::new(Coef::One, v1.ptr()).mul_num(&m, num);
    assert_eq!(term.eval_coef(), 3.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::MinusOne, v1.ptr()).mul_num(&m, num);
    assert_eq!(term.eval_coef(), -3.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::Con(7.0), v2.ptr()).mul_num(&m, num);
    assert_eq!(term.eval_coef(), 21.0);
    assert!(term.var().ptr_eq(v2.ptr()));

    let term = DLinTerm::new(
        Coef::Ptr(m.val::<ValNum>(DNum::new_num(|| 42f32)).ptr()),
        v2.ptr(),
    )
    .mul_num(&m, num);
    assert_eq!(term.eval_coef(), 126.0);
    assert!(term.var().ptr_eq(v2.ptr()));

    // ops - mul con

    let term = DLinTerm::new(Coef::One, v1.ptr()).mul_con(&m, 3);
    assert_eq!(term.eval_coef(), 3.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::MinusOne, v1.ptr()).mul_con(&m, 3);
    assert_eq!(term.eval_coef(), -3.0);
    assert!(term.var().ptr_eq(v1.ptr()));

    let term = DLinTerm::new(Coef::Con(7.0), v2.ptr()).mul_con(&m, 3);
    assert_eq!(term.eval_coef(), 21.0);
    assert!(term.var().ptr_eq(v2.ptr()));

    let term = DLinTerm::new(
        Coef::Ptr(m.val::<ValNum>(DNum::new_num(|| 42f32)).ptr()),
        v2.ptr(),
    )
    .mul_con(&m, 3);
    assert_eq!(term.eval_coef(), 126.0);
    assert!(term.var().ptr_eq(v2.ptr()));
}
