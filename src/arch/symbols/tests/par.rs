use crate::symbols::ParData;
use crate::Model;

fn add_some_more_pars(m: &Model<'_>) {
    for _ in 0..7 {
        let _ = m.par();
    }
}

#[test]
fn par_get_set() {
    let m = Model::new();

    add_some_more_pars(&m);

    let p = m.par();
    assert_eq!(unsafe { p.el.get() }.value(), 0.0);

    add_some_more_pars(&m);

    assert_eq!(unsafe { p.el.get() }.value(), 0.0);

    p.el.set(ParData::new(7.0));
    assert_eq!(unsafe { p.el.get() }.value(), 7.0);
}
