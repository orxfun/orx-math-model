use orx_math_model::*;

// model

struct Mcfp(Model);

impl Mcfp {
    pub fn new() -> Self {
        let model = Model::new();

        let j = model.set().key("j").definition("nodes");
        let i = set_of([j]).key("i").definition("nodes into i");
        let k = set_of([j]).key("k").definition("nodes from i");

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

struct McfpData1 {
    in_nodes: Vec<Vec<usize>>,
    out_nodes: Vec<Vec<usize>>,
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
        out_nodes: vec![vec![1, 2], vec![2, 3], vec![3], vec![]],
    };

    let dj = mcfp.j().data(&data, |d| 0..d.n());
    let di = mcfp.i().data(&data, |d, j| &d.in_nodes[j]);
    let dk = mcfp.k().data(&data, |d, k| &d.out_nodes[k]);

    let data = McfpData2 {
        nodes: vec![
            Node {
                in_nodes: vec![],
                out_nodes: vec![1, 2],
            },
            Node {
                in_nodes: vec![0],
                out_nodes: vec![2, 3],
            },
            Node {
                in_nodes: vec![0],
                out_nodes: vec![3],
            },
            Node {
                in_nodes: vec![1, 2],
                out_nodes: vec![],
            },
        ],
    };

    let dj = mcfp.j().data(&data, |d| 0..d.n());
    let di = mcfp.i().data(&data, |d, j| &d.nodes[j].in_nodes);
    let dk = mcfp.k().data(&data, |d, k| &d.nodes[k].out_nodes);
}
