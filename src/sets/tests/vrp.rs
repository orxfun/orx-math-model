use crate::model::Model;
use crate::sets::{set1_data::Set1Data, Element};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
struct Node {
    id: usize,
    is_depot: bool,
}

impl Element for Node {
    type Idx<'a> = usize;
    fn idx(&self) -> Self::Idx<'_> {
        self.id
    }
}

struct VrpData {
    nodes: Vec<Node>,
}

impl VrpData {
    fn new() -> Self {
        Self {
            nodes: vec![
                Node {
                    id: 0,
                    is_depot: true,
                },
                Node {
                    id: 1,
                    is_depot: false,
                },
                Node {
                    id: 2,
                    is_depot: false,
                },
            ],
        }
    }
}

#[test]
fn vrp_sets() {
    let model = Model::default();

    let nodes = model.set1::<Node>().key("N");
    assert_eq!(nodes.to_string(), "N".to_string());

    // data

    let data = VrpData::new();

    let nodes_data = Set1Data::new(&data, nodes, |d| &d.nodes);
    assert_eq!(nodes_data.values(), &VrpData::new().nodes);
}
