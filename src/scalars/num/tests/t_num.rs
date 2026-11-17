use crate::scalars::num::DNum;
use crate::{Model, SymWrap};

#[test]
fn num() {
    let data = [3.0, 42.0, 2.0];

    // modeling
    let m = Model::default();
    let v1 = m.num(DNum::new_num(|| data[1]));
    let v2 = m.num(DNum::new_num(|| 42f32));
    let v3 = m.num(DNum::new_num(|| 42));
    let v4 = m.num(DNum::new_num(|| 42isize));

    // running
    assert_eq!(v1.data().eval(), 42.0);
    assert_eq!(v2.data().eval(), 42.0);
    assert_eq!(v3.data().eval(), 42.0);
    assert_eq!(v4.data().eval(), 42.0);
}

#[test]
fn int() {
    let data = [3, 42, 2];

    // modeling
    let m = Model::default();
    let v1 = m.int(DNum::new_int(|| data[1]));
    let v2 = m.int(DNum::new_int(|| 42i64));
    let v3 = m.int(DNum::new_int(|| 42usize));
    let v4 = m.int(DNum::new_int(|| 42isize));

    // running
    assert_eq!(v1.data().eval(), 42.0);
    assert_eq!(v2.data().eval(), 42.0);
    assert_eq!(v3.data().eval(), 42.0);
    assert_eq!(v4.data().eval(), 42.0);

    assert_eq!(v1.data().eval_int(), 42);
    assert_eq!(v2.data().eval_int(), 42);
    assert_eq!(v3.data().eval_int(), 42);
    assert_eq!(v4.data().eval_int(), 42);
}
