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
