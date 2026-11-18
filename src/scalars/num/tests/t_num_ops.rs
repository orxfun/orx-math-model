use crate::scalars::num::{DNum, ValInt, ValNum};
use crate::{Model, Num, SymWrap, ValTyp};

#[test]
fn ops_num() {
    let data = [3.0, 1.0, 2.0];

    // modeling
    let m = Model::new();
    let v1 = m.val::<ValNum>(DNum::new_num(|| data[1]));
    let v2 = m.val::<ValNum>(DNum::new_num(|| 2f32));
    let v3 = m.val::<ValNum>(DNum::new_num(|| 3i32));
    let v4 = m.val::<ValNum>(DNum::new_num(|| 4i64));

    // running
    assert_eq!((v1 + v2).data().eval(), 3.0);
    assert_eq!((v2 - v3).data().eval(), -1.0);
    assert_eq!((v3 * v4).data().eval(), 12.0);
    assert_eq!((v4 / v2).data().eval(), 2.0);

    fn num<'m, 'd>(x: Num<'m, 'd>) -> Num<'m, 'd> {
        x
    }

    assert_eq!(num(v1 + 2f32).data().eval(), 3.0);
    assert_eq!(num(2i32 - v3).data().eval(), -1.0);
    assert_eq!(num(v3 * 4i64).data().eval(), 12.0);
    assert_eq!(num(4.0 / v2).data().eval(), 2.0);
}

#[test]
fn ops_int() {
    let data = [3, 1, 2];

    // modeling
    let m = Model::new();
    let v1 = m.val::<ValInt>(DNum::new_int(|| data[1]));
    let v2 = m.val::<ValInt>(DNum::new_int(|| 2i32));
    let v3 = m.val::<ValInt>(DNum::new_int(|| 3i32));
    let v4 = m.val::<ValInt>(DNum::new_int(|| 4i64));

    // running
    assert_eq!((v1 + v2).data().eval_int(), 3);
    assert_eq!((v2 - v3).data().eval_int(), -1);
    assert_eq!((v3 * v4).data().eval_int(), 12);
    assert_eq!((v4 / v2).data().eval(), 2.0);
}
