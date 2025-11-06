use crate::{set_of, Model, SymbolRef};
use alloc::format;

#[test]
fn debug_independent_sets() {
    let m = Model::new();

    let i = m.set();
    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"\", definition: \"\", data: SetData { depends_on: [] } }"
    );

    let i = m.set().key("i");
    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"i\", definition: \"\", data: SetData { depends_on: [] } }"
    );

    let i = m.set().definition("machines");
    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"\", definition: \"machines\", data: SetData { depends_on: [] } }"
    );

    let i = m.set().key("i").definition("machines");
    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"i\", definition: \"machines\", data: SetData { depends_on: [] } }"
    );
}

#[test]
fn debug_dependent_sets() {
    let m = Model::new();
    let i = m.set();
    let j = m.set();

    let k = set_of([i]);
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"\", definition: \"\", data: SetData { depends_on: [0] } }"
    );

    let k = set_of([j]);
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"\", definition: \"\", data: SetData { depends_on: [1] } }"
    );

    let k = set_of([i, j]);
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"\", definition: \"\", data: SetData { depends_on: [0, 1] } }"
    );
}
