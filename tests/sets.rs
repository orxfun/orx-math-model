use orx_math_model::Model;

#[test]
fn sets() {
    let model = Model::new();
    let i = model.set("i");
    let j = i;
    let k = i;
}
