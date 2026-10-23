use crate::symbols::Set;
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
