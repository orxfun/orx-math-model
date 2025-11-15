use crate::model::Model;
use std::string::ToString;

#[test]
fn vrp_sets() {
    #[derive(PartialEq, Eq, Hash)]
    struct Node {
        id: usize,
    }

    let model = Model::default();

    let nodes = model.set1::<Node>().key("N");
    assert_eq!(nodes.to_string(), "N".to_string());
}
