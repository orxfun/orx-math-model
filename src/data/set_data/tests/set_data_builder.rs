use crate::{set_of, Model, SymbolRef};
use alloc::format;

#[test]
fn data_builder_all_set() {
    let m = Model::new();

    let i = m.set();
    let j = m.set();
    let k = set_of(i);

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let builder = m.data_builder().sets((di, dj, dk));
        let data = builder.finish();
        assert!(data.is_ok());
    }

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let builder = m.data_builder().sets((di, dj)).sets(dk);
        let data = builder.finish();
        assert!(data.is_ok());
    }

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let builder = m.data_builder().sets(dk).sets(dj).sets(di);
        let data = builder.finish();
        assert!(data.is_ok());
    }

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);

        let builder = m.data_builder().sets((dj, dk, di));
        let data = builder.finish();
        assert!(data.is_ok());
    }
}

#[test]
fn data_builder_error_on_missing_set() {
    let m = Model::new();

    let i = m.set();
    let j = m.set().key("j");
    let k = set_of(i);

    {
        let di = i.data(&(), |_| 3..7);
        let dk = k.data(&(), |_, i| i..7);

        let builder = m.data_builder().sets((di, dk));
        let data = builder.finish();
        assert!(data.is_err());
        assert_eq!(
            data.err().unwrap(),
            format!("missing data for set with key {}", j)
        );
    }
}

#[test]
fn data_builder_error_double_declaration() {
    let m = Model::new();

    let i = m.set().key("i");
    let j = m.set().key("j");
    let k = set_of(i).key("k");

    {
        let di = i.data(&(), |_| 3..7);
        let dj = j.data(&(), |_| 1..3);
        let dk = k.data(&(), |_, i| i..7);
        let di2 = i.data(&(), |_| 10..13);

        let builder = m.data_builder().sets((di, dj, dk)).sets(di2);
        let data = builder.finish();
        assert!(data.is_err());
        assert_eq!(
            data.err().unwrap(),
            format!("set with key {} is already added", i)
        );
    }
}
