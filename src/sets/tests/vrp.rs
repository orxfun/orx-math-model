use crate::model::Model;
use std::string::ToString;

#[derive(PartialEq, Eq, Hash)]
struct Node {
    id: usize,
    is_depot: bool,
}

#[test]
fn vrp_sets() {
    let model = Model::default();

    let nodes = model.set1::<Node>().key("N");
    assert_eq!(nodes.to_string(), "N".to_string());
}
