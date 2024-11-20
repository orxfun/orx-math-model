use crate::modeling::{
    functions::{forall, sum},
    model::Model,
    model_builder::model_error::ModelError,
    modeler::Modeler,
    stages::Built,
    symbols::{
        conversions::constraint_lhs::ConstraintLhs, data::objective_direction::minimize,
        variable::Variable,
    },
};
use std::{collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct SoaMcnfGraph {
    heads: Vec<Vec<usize>>,
    tails: Vec<Vec<usize>>,
    weights: Vec<HashMap<usize, f32>>,
    capacities: Vec<HashMap<usize, f32>>,
    demands: HashMap<usize, f32>,
}
impl McnfGraph for SoaMcnfGraph {
    fn num_nodes(&self) -> usize {
        self.heads.len()
    }
    fn tails_of(&self, node: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(self.tails[node].iter().cloned())
    }
    fn heads_from(&self, node: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(self.heads[node].iter().cloned())
    }
    fn edge_cost(&self, i: usize, j: usize) -> f32 {
        *self.weights[i].get(&j).unwrap_or(&f32::INFINITY)
    }
    fn edge_capacity(&self, i: usize, j: usize) -> f32 {
        *self.capacities[i].get(&j).unwrap_or(&f32::INFINITY)
    }
    fn node_balance(&self, i: usize) -> f32 {
        *self.demands.get(&i).unwrap_or(&0.0)
    }
}

pub trait McnfGraph {
    fn num_nodes(&self) -> usize;
    fn tails_of(&self, node: usize) -> Box<dyn Iterator<Item = usize> + '_>;
    fn heads_from(&self, node: usize) -> Box<dyn Iterator<Item = usize> + '_>;
    fn edge_cost(&self, i: usize, j: usize) -> f32;
    fn edge_capacity(&self, i: usize, j: usize) -> f32;
    fn node_balance(&self, i: usize) -> f32;
}

pub struct Mcnf<G: McnfGraph> {
    pub graph: Rc<G>,
    pub model: Model,
    pub x: Variable<2, Built>,
}

impl<G: McnfGraph + 'static> Mcnf<G> {
    #[allow(clippy::precedence)]
    pub fn new(graph: Rc<G>) -> Result<Self, ModelError> {
        let m = Modeler::new();

        let n = graph.num_nodes();
        let graph = || graph.clone();

        let j = m.set(("j", "node")).from_range(0..n);

        let k = m
            .set(("k", "heads of j"))
            .depends_on(j)
            .from_data_and_fun(graph(), |gr, j| gr.tails_of(j));

        let i = m
            .set(("i", "tails from j"))
            .depends_on(j)
            .from_data_and_fun(graph(), |gr, j| gr.heads_from(j));

        let cost = m
            .parameter(("cost", "unit cost of flow on edge (j, k)"))
            .has_dim2()
            .from_data_and_fun(graph(), |gr, [i, j]| gr.edge_cost(i, j));

        let d = m
            .parameter(("d", "flow balance at node j"))
            .has_dim1()
            .from_data_and_fun(graph(), |gr, [i]| gr.node_balance(i));

        let cap = m
            .parameter(("cap", "capacity of edge (j, k)"))
            .has_dim2()
            .from_data_and_fun(graph(), |gr, [i, j]| gr.edge_capacity(i, j));

        let x = m
            .variable(("x", "flow on edge (j, k)"))
            .has_dim2()
            .is_continuous()
            .is_nonnegative_with_param_ub(cap);

        let con_capacity = x[(j, k)] << cap[(j, k)] | forall((j, k));
        let con_flow_bal = sum(k | x[(j, k)]).equal_to(sum(i | x[(i, j)]) + d[j]) | forall(j);
        let total_cost = sum((j, k) | cost[(j, k)] * x[(j, k)]);

        let definition = m
            .define()
            .constraints([con_capacity, con_flow_bal])
            .objective(minimize | total_cost);

        let x = definition.cache_var_dim2(x)?;
        let model = definition.build()?;

        Ok(Self {
            graph: graph(),
            model,
            x,
        })
    }
}
