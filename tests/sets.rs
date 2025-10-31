use orx_math_model::*;

#[test]
fn sets() {
    let model = Model::new();

    let i = model.set("i");
    assert_eq!(format!("{i:?}"), "Set { key: \"i\", definition: \"\" }");

    let j = model.set("j").definition("vehicles");
    assert_eq!(
        format!("{j:?}"),
        "Set { key: \"j\", definition: \"vehicles\" }"
    );

    let k = model.set("k").definition("vehicles").definition("tasks");
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"k\", definition: \"tasks\" }"
    );
}

#[test]
fn set_reference_equality() {
    // same model
    let m1 = Model::new();
    let i = m1.set("i");
    let i2 = i;
    let j = m1.set("i");
    assert_eq!(i, i2);
    assert_ne!(i, j);
    assert_ne!(i2, j);

    // different models
    let m2 = Model::new();
    let k = m2.set("k");
    assert_ne!(k, i);
    assert_ne!(k, i2);
    assert_ne!(k, j);
}
