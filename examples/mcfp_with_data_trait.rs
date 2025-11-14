use orx_math_model::*;
use orx_self_or::SoR;
use std::collections::HashMap;

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

    pub fn j(&self) -> Set<'_, 0> {
        self.0.set_by_key("j").unwrap()
    }

    pub fn k(&self) -> Set<'_, 1> {
        self.0.set_by_key("k").unwrap()
    }

    pub fn c(&self) -> Par<'_, 2> {
        self.0.par_by_key("c").unwrap()
    }

    pub fn b(&self) -> Par<'_, 2> {
        self.0.par_by_key("b").unwrap()
    }

    pub fn build_data<'m, D: McfpData>(&'m self, data: &'m D) -> Data<'m> {
        let dj = self.j().data(data, D::j);
        let di = self.i().data(data, D::i);
        let dk = self.k().data(data, D::k);

        let (c, b) = (data.c(self), data.b(self));
        let builder = self.0.data_builder().sets((di, dj, dk)).pars((c, b));
        builder.finish().unwrap()
    }
}

// data trait

trait McfpData {
    // sets

    fn j<'m>(&'m self) -> impl IntoIterator<Item = impl SoR<usize>>;

    fn k<'m>(&'m self, j: usize) -> impl IntoIterator<Item = impl SoR<usize>>;

    fn i<'m>(&'m self, j: usize) -> impl IntoIterator<Item = impl SoR<usize>>;

    // pars

    fn c<'m>(&'m self, m: &'m Mcfp) -> impl ParData<'m, 2>;

    fn b<'m>(&'m self, m: &'m Mcfp) -> impl ParData<'m, 2>;
}

// # 1: data implementation

struct Edge {
    cost: u64,
    cap: u64,
}

struct McfpData1 {
    in_nodes: Vec<Vec<usize>>,
    out_nodes: Vec<HashMap<usize, Edge>>,
}

impl McfpData for McfpData1 {
    fn j<'m>(&'m self) -> impl IntoIterator<Item = impl SoR<usize>> {
        0..self.in_nodes.len()
    }

    fn k<'m>(&'m self, j: usize) -> impl IntoIterator<Item = impl SoR<usize>> {
        self.out_nodes[j].iter().map(|(head, _)| *head)
    }

    fn i<'m>(&'m self, j: usize) -> impl IntoIterator<Item = impl SoR<usize>> {
        &self.in_nodes[j]
    }

    fn c<'m>(&'m self, m: &'m Mcfp) -> impl ParData<'m, 2> {
        m.c().data(self, |d, j, k| d.out_nodes[j][&k].cost)
    }

    fn b<'m>(&'m self, m: &'m Mcfp) -> impl ParData<'m, 2> {
        m.b().data(self, |d, j, k| d.out_nodes[j][&k].cap)
    }
}

// data

struct Node {
    in_nodes: Vec<usize>,
    out_nodes: Vec<usize>,
    edges: Vec<Edge>,
}

struct McfpData2 {
    nodes: Vec<Node>,
}

impl McfpData for McfpData2 {
    fn j<'m>(&'m self) -> impl IntoIterator<Item = impl SoR<usize>> {
        0..self.nodes.len()
    }

    fn k<'m>(&'m self, j: usize) -> impl IntoIterator<Item = impl SoR<usize>> {
        &self.nodes[j].out_nodes
    }

    fn i<'m>(&'m self, j: usize) -> impl IntoIterator<Item = impl SoR<usize>> {
        &self.nodes[j].in_nodes
    }

    fn c<'m>(&'m self, m: &'m Mcfp) -> impl ParData<'m, 2> {
        m.c().data(self, |d, j, k| d.nodes[j].edges[k].cost)
    }

    fn b<'m>(&'m self, m: &'m Mcfp) -> impl ParData<'m, 2> {
        m.b().data(self, |d, j, k| d.nodes[j].edges[k].cap)
    }
}

fn main() {
    let mcfp = Mcfp::new();

    let data_source = McfpData1 {
        in_nodes: vec![vec![], vec![0], vec![0], vec![1, 2]],
        out_nodes: vec![
            [(1, Edge { cost: 1, cap: 3 }), (2, Edge { cost: 3, cap: 5 })]
                .into_iter()
                .collect(),
            [(2, Edge { cost: 7, cap: 5 }), (3, Edge { cost: 4, cap: 4 })]
                .into_iter()
                .collect(),
            [(3, Edge { cost: 2, cap: 9 })].into_iter().collect(),
            [].into_iter().collect(),
        ],
    };
    let data = mcfp.build_data(&data_source);

    let inf_edge = || Edge {
        cost: u64::MAX,
        cap: 0,
    };

    let data_source = McfpData2 {
        nodes: vec![
            Node {
                in_nodes: vec![],
                out_nodes: vec![1, 2],
                edges: vec![
                    inf_edge(),
                    Edge { cost: 1, cap: 3 },
                    Edge { cost: 3, cap: 5 },
                    inf_edge(),
                ],
            },
            Node {
                in_nodes: vec![0],
                out_nodes: vec![2, 3],
                edges: vec![
                    inf_edge(),
                    inf_edge(),
                    Edge { cost: 7, cap: 5 },
                    Edge { cost: 4, cap: 4 },
                ],
            },
            Node {
                in_nodes: vec![0],
                out_nodes: vec![3],
                edges: vec![inf_edge(), inf_edge(), inf_edge(), Edge { cost: 2, cap: 9 }],
            },
            Node {
                in_nodes: vec![1, 2],
                out_nodes: vec![],
                edges: vec![inf_edge(), inf_edge(), inf_edge(), inf_edge()],
            },
        ],
    };
    let data = mcfp.build_data(&data_source);
}
