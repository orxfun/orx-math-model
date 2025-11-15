use crate::{set_of, Model, SymbolRef};
use alloc::format;

#[test]
fn par_data_builder_all_set() {
    let m = Model::new();

    let i = m.set();
    let j = m.set();
    let k = set_of(i);
    let s = m.par();
    let t = m.par_of(i);
    let u = m.par_of((i, j, k));

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let ds = s.data(&(), |_| 1);
        let dt = t.data(&(), |_, i| i);
        let du = u.data(&(), |_, i, j, k| i + j + k);

        let builder = m.data_builder().sets((di, dj, dk)).pars((ds, dt, du));
        let data = builder.finish();
        assert!(data.is_ok());
    }

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let ds = s.data(&(), |_| 1);
        let dt = t.data(&(), |_, i| i);
        let du = u.data(&(), |_, i, j, k| i + j + k);

        let builder = m
            .data_builder()
            .sets((di, dj))
            .sets(dk)
            .pars(ds)
            .pars((dt, du));
        let data = builder.finish();
        assert!(data.is_ok());
    }

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let ds = s.data(&(), |_| 1);
        let dt = t.data(&(), |_, i| i);
        let du = u.data(&(), |_, i, j, k| i + j + k);

        let builder = m
            .data_builder()
            .sets(dk)
            .sets(dj)
            .sets(di)
            .pars(ds)
            .pars(du)
            .pars(dt);
        let data = builder.finish();
        assert!(data.is_ok());
    }

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let ds = s.data(&(), |_| 1);
        let dt = t.data(&(), |_, i| i);
        let du = u.data(&(), |_, i, j, k| i + j + k);

        let builder = m.data_builder().sets((dj, dk, di)).pars((du, ds, dt));
        let data = builder.finish();
        assert!(data.is_ok());
    }
}

#[test]
fn par_data_builder_error_on_missing_par() {
    let m = Model::new();

    let i = m.set();
    let j = m.set();
    let k = set_of(i);
    let s = m.par();
    let t = m.par_of(i).key("t");
    let u = m.par_of((i, j, k));

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let ds = s.data(&(), |_| 1);
        let du = u.data(&(), |_, i, j, k| i + j + k);

        let builder = m.data_builder().sets((di, dk, dj)).pars((ds, du));
        let data = builder.finish();
        assert!(data.is_err());
        assert_eq!(
            data.err().unwrap(),
            format!("missing data for par with key {t}")
        );
    }
}

#[test]
fn par_data_builder_error_double_declaration() {
    let m = Model::new();

    let i = m.set().key("i");
    let j = m.set().key("j");
    let k = set_of(i).key("k");
    let s = m.par();
    let t = m.par_of(i);
    let u = m.par_of((i, j, k)).key("u");

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);
        let ds = s.data(&(), |_| 1);
        let dt = t.data(&(), |_, i| i);
        let du = u.data(&(), |_, i, j, k| i + j + k);
        let du2 = u.data(&(), |_, i, j, k| i + j + k);

        let builder = m.data_builder().sets((di, dj, dk)).pars((ds, dt, du, du2));
        let data = builder.finish();
        assert!(data.is_err());
        assert_eq!(
            data.err().unwrap(),
            format!("double data definition for par with key {u}")
        );
    }

    {
        let dk = k.data(&(), |_, i| i..7);
        let dk2 = k.data(&(), |_, i| i..7);

        let builder = m.data_builder().sets((dk2, dk));
        let data = builder.finish();
        assert!(data.is_err());
        assert_eq!(
            data.err().unwrap(),
            format!("double data definition for set with key {k}")
        );
    }
}
