use crate::symbols::{ParData, Sca};
use crate::Model;

#[test]
fn sca_get_add() {
    let m = Model::new();

    let p0 = m.par();
    p0.el.set(ParData::new(7.0));

    let p1 = m.par();
    p1.el.set(ParData::new(42.0));

    let s0 = Sca::from(p0);
    assert_eq!(unsafe { s0.el.get() }.eval(), 7.0);

    let s1 = Sca::from(p1);
    assert_eq!(unsafe { s1.el.get() }.eval(), 42.0);

    let s2 = s0 + s1;
    assert_eq!(unsafe { s2.el.get() }.eval(), 49.0);

    let s3 = s0 + 2.0;
    assert_eq!(unsafe { s3.el.get() }.eval(), 9.0);

    let s4 = 12.0 + s2;
    assert_eq!(unsafe { s4.el.get() }.eval(), 61.0);
}
