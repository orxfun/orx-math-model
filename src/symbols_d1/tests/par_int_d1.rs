use crate::symbols::Int;
use crate::symbols_d1::{ParIntD1, ParIntD1Fun};
use crate::Model;
use alloc::boxed::Box;
use alloc::vec;

fn add_some_more_pars(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.par_int_d1(m.set_of_idx());
    }
}

#[test]
fn par_int_d1_get_set() {
    fn assert_values(par_symbols: &[ParIntD1<'_, '_>], i0: usize, expected: usize) {
        for s in par_symbols {
            assert_eq!(unsafe { s.el.get() }.eval(i0), expected);
        }
    }

    let data = vec![10, 11, 12, 13];

    let m = Model::new();
    add_some_more_pars(&m);

    let s0 = m.set_of_idx();
    let par = m.par_int_d1(s0);
    let p = par;

    assert_values(&[p, par], 0, 0);
    assert_values(&[p, par], 42, 0);

    p.el.update(|x| x.update_fun(ParIntD1Fun::new(Box::new(|_| 42))));
    assert_values(&[p, par], 0, 42);
    assert_values(&[p, par], 42, 42);

    p.el.update(|x| x.update_fun(ParIntD1Fun::new(Box::new(|i| i as usize))));
    assert_values(&[p, par], 0, 0);
    assert_values(&[p, par], 42, 42);

    p.el.update(|x| x.update_fun(ParIntD1Fun::new(Box::new(|i| data[i]))));
    assert_values(&[p, par], 0, 10);
    assert_values(&[p, par], 1, 11);
    assert_values(&[p, par], 2, 12);
    assert_values(&[p, par], 3, 13);
}

#[test]
fn sca_from_par_int_d1() {
    fn assert_value(sca: Int<'_, '_>, expected: usize) {
        assert_eq!(unsafe { sca.el.get() }.eval(), expected);
    }

    let data = vec![10, 11, 12, 13];

    let m = Model::new();
    add_some_more_pars(&m);

    let s0 = m.set_of_idx();
    let p1 = m.par_int_d1(s0);

    let i0 = s0.idx();
    let p = p1.at(i0);

    // before wire up par-d1 data

    assert_value(p, 0);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 0);

    // after wire up par-d1 data

    p1.el
        .update(|x| x.update_fun(ParIntD1Fun::new(Box::new(|i| data[i]))));

    i0.el.update(|x| x.set_value(0));
    assert_value(p, 10);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 11);

    i0.el.update(|x| x.set_value(2));
    assert_value(p, 12);

    i0.el.update(|x| x.set_value(3));
    assert_value(p, 13);

    // int expression as index

    let p = p1.at(i0 + 1);

    i0.el.update(|x| x.set_value(0));
    assert_value(p, 11);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 12);

    i0.el.update(|x| x.set_value(2));
    assert_value(p, 13);

    // constant int as index

    let p = p1.at(2);
    assert_value(p, 12);
}
