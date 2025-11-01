use crate::Model;
use crate::Sym;
use alloc::format;
use alloc::vec;

#[test]
fn sets() {
    let m = Model::new();

    let i = m.set();
    assert_eq!(
        format!("{i:?}"),
        "Set { kind: Independent, key: \"\", definition: \"\" }"
    );

    let i = m.set().key("i");
    assert_eq!(
        format!("{i:?}"),
        "Set { kind: Independent, key: \"i\", definition: \"\" }"
    );

    let i = m.set().definition("machines");
    assert_eq!(
        format!("{i:?}"),
        "Set { kind: Independent, key: \"\", definition: \"machines\" }"
    );

    let i = m.set().key("i").definition("machines");
    assert_eq!(
        format!("{i:?}"),
        "Set { kind: Independent, key: \"i\", definition: \"machines\" }"
    );
}
