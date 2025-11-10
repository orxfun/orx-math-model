use orx_math_model::*;

// model

struct Knapsack(Model);

impl Knapsack {
    fn new() -> Self {
        let model = Model::new();

        let i = model.set().key("i").def("items");
        let w = par(i).key("w").def("weight of item i");
        let u = par(i).key("u").def("utilization of item i");

        let c = model.scalar().key("C").def("knapsack capacity");

        Self(model)
    }

    fn i(&self) -> Set<'_> {
        self.0.set_by_key("i").unwrap()
    }
}

// data

struct KnapsackData1 {
    costs: Vec<u64>,
    weights: Vec<u64>,
    knapsack_capacity: u64,
}

impl KnapsackData1 {
    fn data<'m>(&'m self, knapsack: &'m Knapsack) -> Data<'m> {
        let num_items = self.costs.len();
        let di = knapsack.i().data(self, move |_| 0..num_items);
        knapsack.0.data_builder().sets(di).finish().unwrap()
    }
}

// data

struct Item {
    cost: u64,
    weight: u64,
}

struct KnapsackData2 {
    items: Vec<Item>,
    knapsack_capacity: u64,
}

impl KnapsackData2 {
    fn num_items(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let knapsack = Knapsack::new();

    let data_source = KnapsackData1 {
        costs: vec![3, 1, 7, 6],
        weights: vec![2, 5, 4, 6],
        knapsack_capacity: 11,
    };
    let data = data_source.data(&knapsack);

    let data = KnapsackData2 {
        items: vec![
            Item { cost: 3, weight: 2 },
            Item { cost: 1, weight: 5 },
            Item { cost: 7, weight: 4 },
            Item { cost: 6, weight: 6 },
        ],
        knapsack_capacity: 11,
    };
    let di = knapsack.i().data(&data, |d| 0..d.num_items());
}
