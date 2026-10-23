use crate::symbols::{Sca, Set};
use crate::symbols_d1::{ParD1, ParD1Fun, SetD1};
use crate::Model;
use alloc::boxed::Box;
use alloc::vec;

fn add_some_more_pars(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.par_d1(m.set());
    }
}

#[test]
fn par_d1_get_set() {
    fn assert_values(par_symbols: &[ParD1<'_, '_>], i0: usize, expected: f64) {
        for s in par_symbols {
            assert_eq!(unsafe { s.el.get() }.eval(i0), expected);
        }
    }

    let data = vec![10.0, 11.0, 12.0, 13.0];

    let m = Model::new();
    add_some_more_pars(&m);

    let s0 = m.set();
    let par = m.par_d1(s0);
    let p = par;

    assert_values(&[p, par], 0, 0.0);
    assert_values(&[p, par], 42, 0.0);

    p.el.update(|x| x.update_fun(ParD1Fun::new(Box::new(|_| 42.0))));
    assert_values(&[p, par], 0, 42.0);
    assert_values(&[p, par], 42, 42.0);

    p.el.update(|x| x.update_fun(ParD1Fun::new(Box::new(|i| i as f64))));
    assert_values(&[p, par], 0, 0.0);
    assert_values(&[p, par], 42, 42.0);

    p.el.update(|x| x.update_fun(ParD1Fun::new(Box::new(|i| data[i]))));
    assert_values(&[p, par], 0, 10.0);
    assert_values(&[p, par], 1, 11.0);
    assert_values(&[p, par], 2, 12.0);
    assert_values(&[p, par], 3, 13.0);
}

#[test]
fn sca_from_par_d1() {
    fn assert_value(sca: Sca<'_, '_>, expected: f64) {
        assert_eq!(unsafe { sca.el.get() }.eval(), expected);
    }

    let data = vec![10.0, 11.0, 12.0, 13.0];

    let m = Model::new();
    add_some_more_pars(&m);

    let s0 = m.set();
    let p1 = m.par_d1(s0);

    let i0 = s0.idx();
    let p = p1.sca_at(i0);

    // before wire up par-d1 data

    assert_value(p, 0.0);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 0.0);

    // after wire up par-d1 data

    p1.el
        .update(|x| x.update_fun(ParD1Fun::new(Box::new(|i| data[i]))));

    i0.el.update(|x| x.set_value(0));
    assert_value(p, 10.0);

    i0.el.update(|x| x.set_value(1));
    assert_value(p, 11.0);

    i0.el.update(|x| x.set_value(2));
    assert_value(p, 12.0);

    i0.el.update(|x| x.set_value(3));
    assert_value(p, 13.0);
}

#[test]
#[should_panic]
fn sca_from_par_d1_wrong_idx() {
    let m = Model::new();
    let s0 = m.set();
    let p1 = m.par_d1(s0);

    let wrong_set = m.set();
    let wrong_idx = wrong_set.idx();

    let _ = p1.sca_at(wrong_idx);
}
