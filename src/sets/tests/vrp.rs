use crate::{model::Model, sets::Element};
use std::string::ToString;

struct Node {
    id: usize,
    is_depot: bool,
}

impl Element for Node {
    type Idx = usize;
    fn idx(&self) -> Self::Idx {
        self.id
    }
}

#[test]
fn vrp_sets() {
    let model = Model::default();

    let nodes = model.set1::<Node>().key("N");
    assert_eq!(nodes.to_string(), "N".to_string());
}
