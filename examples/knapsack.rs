use orx_math_model::*;

// model

struct Knapsack(Model);

impl Knapsack {
    fn new() -> Self {
        let model = Model::new();

        let i = model.set().key("i");

        Self(model)
    }

    fn i(&self) -> Set<'_> {
        self.0.set_by_key("i").unwrap()
    }
}

// data

struct KnapsackData {
    costs: Vec<u64>,
    weights: Vec<u64>,
    knapsack_capacity: u64,
}

impl KnapsackData {
    fn num_items(&self) -> usize {
        self.costs.len()
    }
}

fn main() {
    let knapsack = Knapsack::new();

    let data = KnapsackData {
        costs: vec![3, 1, 7, 6],
        weights: vec![2, 5, 4, 6],
        knapsack_capacity: 11,
    };

    let di = knapsack.i().data(&data, |d| 0..d.num_items());

    drop(knapsack);
}
