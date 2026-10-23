use crate::Model;

#[test]
fn read_write_par_d0() {
    let m = Model::new();

    for _ in 0..3 {
        let _ = m.par_d0();
    }

    let p = m.par_d0();
    assert_eq!(p.value(), 0);

    for _ in 0..3 {
        let _ = m.par_d0();
    }

    p.set(42);
    assert_eq!(p.value(), 42);
}

sdf