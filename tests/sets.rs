use orx_math_model::Model;

#[test]
fn sets() {
    let a = 12;
    let model = Model::new();

    let i = model.set("i");
    assert_eq!(format!("{i:?}"), "Set { key: \"i\", definition: \"\" }");

    // let j = model.set("j").definition("vehicles");
    // assert_eq!(format!("{j:?}"), "Set { key: \"i\", definition: \"\" }");

    assert_eq!(a, 33);
}
