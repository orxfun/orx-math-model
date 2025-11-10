use crate::{set_of, Model, SymbolRef};
use alloc::format;

#[test]
fn debug_independent_sets() {
    let m = Model::new();

    let i = m.set();

    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"\", definition: \"\", depending_sets: [] }"
    );

    let i = m.set().key("i");
    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"i\", definition: \"\", depending_sets: [] }"
    );

    let i = m.set().def("machines");
    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"\", definition: \"machines\", depending_sets: [] }"
    );

    let i = m.set().key("i").def("machines");
    assert_eq!(
        format!("{i:?}"),
        "Set { key: \"i\", definition: \"machines\", depending_sets: [] }"
    );
}

#[test]
fn debug_dependent_sets() {
    let m = Model::new();
    let i = m.set().key("i");
    let j = m.set().key("j");

    let k = set_of(i).key("k");
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"k\", definition: \"\", depending_sets: [Set { key: \"i\", definition: \"\", depending_sets: [] }] }"
    );

    let k = set_of(j).key("k");
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"k\", definition: \"\", depending_sets: [Set { key: \"j\", definition: \"\", depending_sets: [] }] }"
    );

    let k = set_of((i, j)).key("k");
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"k\", definition: \"\", depending_sets: [Set { key: \"i\", definition: \"\", depending_sets: [] }, Set { key: \"j\", definition: \"\", depending_sets: [] }] }"
    );
}
