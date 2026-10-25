use crate::symbols::Num;
use crate::symbols_d1::{ParD1Depr, ParD1FunDepr};
use crate::Model;
use alloc::boxed::Box;
use alloc::vec;

fn add_some_more_pars(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.par_d1(m.indices());
    }
}

#[test]
fn par_d1_get_set() {
    fn assert_values(par_symbols: &[ParD1Depr<'_, '_>], i0: usize, expected: f64) {
        for s in par_symbols {
            assert_eq!(unsafe { s.el.get() }.eval(i0), expected);
        }
    }

    let data = vec![10.0, 11.0, 12.0, 13.0];

    let m = Model::new();
    add_some_more_pars(&m);

    let s0 = m.indices();
    let par = m.par_d1(s0);
    let p = par;

    assert_values(&[p, par], 0, 0.0);
    assert_values(&[p, par], 42, 0.0);

    p.el.update(|x| x.update_fun(ParD1FunDepr::new(Box::new(|_| 42.0))));
    assert_values(&[p, par], 0, 42.0);
    assert_values(&[p, par], 42, 42.0);

    p.el.update(|x| x.update_fun(ParD1FunDepr::new(Box::new(|i| i as f64))));
    assert_values(&[p, par], 0, 0.0);
    assert_values(&[p, par], 42, 42.0);

    p.el.update(|x| x.update_fun(ParD1FunDepr::new(Box::new(|i| data[i]))));
    assert_values(&[p, par], 0, 10.0);
    assert_values(&[p, par], 1, 11.0);
    assert_values(&[p, par], 2, 12.0);
    assert_values(&[p, par], 3, 13.0);
}

#[test]
fn sca_from_par_d1() {
    fn assert_value(sca: Num<'_, '_>, expected: f64) {
        assert_eq!(unsafe { sca.el.get() }.eval(), expected);
    }

    let data = vec![10.0, 11.0, 12.0, 13.0];

    let m = Model::new();
    add_some_more_pars(&m);

    let s0 = m.indices();
    let p1 = m.par_d1(s0);

    let i0 = s0.idx();
    let p = p1.at(i0);

    // before wire up par-d1 data

    assert_value(p, 0.0);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 0.0);

    // after wire up par-d1 data

    p1.el
        .update(|x| x.update_fun(ParD1FunDepr::new(Box::new(|i| data[i]))));

    i0.el.update(|x| x.set_value(0));
    assert_value(p, 10.0);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 11.0);

    i0.el.update(|x| x.set_value(2));
    assert_value(p, 12.0);

    i0.el.update(|x| x.set_value(3));
    assert_value(p, 13.0);

    // int expression as index

    let p = p1.at(i0 + 1);

    i0.el.update(|x| x.set_value(0));
    assert_value(p, 11.0);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 12.0);

    i0.el.update(|x| x.set_value(2));
    assert_value(p, 13.0);

    // constant int as index

    let p = p1.at(2);
    assert_value(p, 12.0);
}
