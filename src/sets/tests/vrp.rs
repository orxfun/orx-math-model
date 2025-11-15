use crate::model::Model;
use crate::sets::{set1_data::Set1Data, Element};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
struct Node {
    id: usize,
    is_depot: bool,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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

    let n = model.set1::<Node>().key("N");
    let n0 = n.st(|x| !x.is_depot);
    let n0_b = n | |x| !x.is_depot;
    let e_c = n * n;
    let e = (n * n).st(|a, b| a != b);
    let e_b = n * n | |a, b| a != b;

    // data

    let data = VrpData::new();

    let nodes_data = Set1Data::new(&data, n, |d| &d.nodes);
    assert_eq!(&VrpData::new().nodes, nodes_data.values());

    assert_eq!(
        VrpData::new().nodes.iter().skip(1).collect::<Vec<_>>(),
        n0.values(&nodes_data).collect::<Vec<_>>()
    );
    assert_eq!(
        VrpData::new().nodes.iter().skip(1).collect::<Vec<_>>(),
        n0_b.values(&nodes_data).collect::<Vec<_>>()
    );

    let edge_indices: Vec<_> = e_c
        .values(&nodes_data, &nodes_data)
        .map(|(a, b)| (a.id, b.id))
        .collect();
    assert_eq!(
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ],
        edge_indices
    );

    let edge_indices: Vec<_> = e
        .values(&nodes_data, &nodes_data)
        .map(|(a, b)| (a.id, b.id))
        .collect();
    assert_eq!(
        vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)],
        edge_indices
    );

    let edge_indices: Vec<_> = e_b
        .values(&nodes_data, &nodes_data)
        .map(|(a, b)| (a.id, b.id))
        .collect();
    assert_eq!(
        vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)],
        edge_indices
    );
}
