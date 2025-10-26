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
