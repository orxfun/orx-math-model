use crate::model::Model;
use crate::sets::subset2::Subset2;
use crate::sets::{set1_data::Set1Data, Element};
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
    let nodes0_a = nodes.st(|x| !x.is_depot);
    let nodes0_b = nodes | |x| !x.is_depot;
    let edges = Subset2::new(nodes, nodes, |a, b| a.id != b.id);

    // data

    let data = VrpData::new();

    let nodes_data = Set1Data::new(&data, nodes, |d| &d.nodes);
    assert_eq!(&VrpData::new().nodes, nodes_data.values());

    assert_eq!(
        VrpData::new().nodes.iter().skip(1).collect::<Vec<_>>(),
        nodes0_a.values(&nodes_data).collect::<Vec<_>>()
    );
    assert_eq!(
        VrpData::new().nodes.iter().skip(1).collect::<Vec<_>>(),
        nodes0_b.values(&nodes_data).collect::<Vec<_>>()
    );

    let edge_indices: Vec<_> = edges
        .values(&nodes_data, &nodes_data)
        .map(|(a, b)| (a.id, b.id))
        .collect();
    assert_eq!(
        vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)],
        edge_indices
    );
}
