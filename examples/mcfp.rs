use orx_math_model::*;

// model

struct Mcfp(Model);

impl Mcfp {
    pub fn new() -> Self {
        let model = Model::new();

        let j = model.set().key("j").def("nodes");
        let i = set_of(j).key("i").def("nodes into i");
        let k = set_of(j).key("k").def("nodes from i");

        let c = par((j, k)).key("c").def("unit cost of using arc (j, k)");
        let b = par((j, k)).key("b").def("capacity of arc (j, k)");

        Self(model)
    }

    pub fn i(&self) -> Set<'_, 1> {
        self.0.set_by_key("i").unwrap()
    }

    pub fn j(&self) -> Set<'_> {
        self.0.set_by_key("j").unwrap()
    }

    pub fn k(&self) -> Set<'_, 1> {
        self.0.set_by_key("k").unwrap()
    }
}

// data

struct Edge {
    cost: u64,
    cap: u64,
}

struct McfpData1 {
    in_nodes: Vec<Vec<usize>>,
    out_nodes: Vec<Vec<(usize, Edge)>>,
}

impl McfpData1 {
    fn n(&self) -> usize {
        self.in_nodes.len()
    }
}

// data

struct Node {
    in_nodes: Vec<usize>,
    out_nodes: Vec<usize>,
    out_edges: Vec<Edge>,
}

struct McfpData2 {
    nodes: Vec<Node>,
}

impl McfpData2 {
    fn n(&self) -> usize {
        self.nodes.len()
    }
}

fn main() {
    let mcfp = Mcfp::new();

    let data = McfpData1 {
        in_nodes: vec![vec![], vec![0], vec![0], vec![1, 2]],
        out_nodes: vec![
            vec![(1, Edge { cost: 1, cap: 3 }), (2, Edge { cost: 3, cap: 5 })],
            vec![(2, Edge { cost: 7, cap: 5 }), (3, Edge { cost: 4, cap: 4 })],
            vec![(3, Edge { cost: 2, cap: 9 })],
            vec![],
        ],
    };

    let dj = mcfp.j().data(&data, |d| 0..d.n());
    let di = mcfp.i().data(&data, |d, j| &d.in_nodes[j]);
    let dk = mcfp
        .k()
        .data(&data, |d, k| d.out_nodes[k].iter().map(|(head, _)| *head));

    let data = McfpData2 {
        nodes: vec![
            Node {
                in_nodes: vec![],
                out_nodes: vec![1, 2],
                out_edges: vec![Edge { cost: 1, cap: 3 }, Edge { cost: 3, cap: 5 }],
            },
            Node {
                in_nodes: vec![0],
                out_nodes: vec![2, 3],
                out_edges: vec![Edge { cost: 7, cap: 5 }, Edge { cost: 4, cap: 4 }],
            },
            Node {
                in_nodes: vec![0],
                out_nodes: vec![3],
                out_edges: vec![Edge { cost: 2, cap: 9 }],
            },
            Node {
                in_nodes: vec![1, 2],
                out_nodes: vec![],
                out_edges: vec![],
            },
        ],
    };

    let dj = mcfp.j().data(&data, |d| 0..d.n());
    let di = mcfp.i().data(&data, |d, j| &d.nodes[j].in_nodes);
    let dk = mcfp.k().data(&data, |d, k| &d.nodes[k].out_nodes);
}
