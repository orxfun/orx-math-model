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

struct McfpData {
    in_nodes: Vec<Vec<usize>>,
    out_nodes: Vec<Vec<usize>>,
}

impl McfpData {
    fn n(&self) -> usize {
        self.in_nodes.len()
    }
}

fn main() {
    let mcfp = Mcfp::new();

    let data = McfpData {
        in_nodes: vec![vec![], vec![0], vec![0], vec![1, 2]],
        out_nodes: vec![vec![1, 2], vec![3], vec![3], vec![]],
    };

    let dj = mcfp.j().data(&data, |d| 0..d.n());
    let di = mcfp.i().data(&data, |d, j| &d.in_nodes[j]);
    let dk = mcfp.k().data(&data, |d, k| &d.out_nodes[k]);
}
