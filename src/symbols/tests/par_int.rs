use crate::symbols::ParIntData;
use crate::Model;

fn add_some_more_pars(m: &Model<'_>) {
    for _ in 0..7 {
        let _ = m.par_int();
    }
}

#[test]
fn par_get_set() {
    let m = Model::new();

    add_some_more_pars(&m);

    let p = m.par_int();
    assert_eq!(unsafe { p.el.get() }.value(), 0);

    add_some_more_pars(&m);

    assert_eq!(unsafe { p.el.get() }.value(), 0);

    p.el.set(ParIntData::new(7));
    assert_eq!(unsafe { p.el.get() }.value(), 7);
}
