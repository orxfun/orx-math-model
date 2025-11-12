use orx_math_model::*;

// model

struct Knapsack(Model);

impl Knapsack {
    fn new() -> Self {
        let model = Model::new();

        let i = model.set().key("i").def("items");
        let w = par(i).key("w").def("weight of item i");
        let u = par(i).key("u").def("utilization of item i");

        let c = model.par().key("C").def("knapsack capacity");

        Self(model)
    }

    fn i(&self) -> Set<'_> {
        self.0.set_by_key("i").unwrap()
    }

    fn w(&self) -> Par<'_, 1> {
        self.0.par_by_key("w").unwrap()
    }

    fn u(&self) -> Par<'_, 1> {
        self.0.par_by_key("u").unwrap()
    }

    fn c(&self) -> Par<'_, 0> {
        self.0.par_by_key("C").unwrap()
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
        let di = knapsack.i().data(self, |d| 0..d.costs.len());

        let dw = knapsack.w().data(self, |d, i| d.weights[i]);
        let du = knapsack.u().data(self, |d, i| -(d.costs[i] as i64));
        let dc = knapsack.c().data(self, |d| d.knapsack_capacity);

        let builder = knapsack.0.data_builder().sets(di).pars((dw, du, dc));
        builder.finish().unwrap()
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
    fn data<'m>(&'m self, knapsack: &'m Knapsack) -> Data<'m> {
        let di = knapsack.i().data(self, move |d| 0..d.items.len());

        let dw = knapsack.w().data(self, |d, i| d.items[i].weight);
        let du = knapsack.u().data(self, |d, i| -(d.items[i].cost as i64));
        let dc = knapsack.c().data(self, |d| d.knapsack_capacity);

        let builder = knapsack.0.data_builder().sets(di).pars((dw, du, dc));
        builder.finish().unwrap()
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

    let data_source = KnapsackData2 {
        items: vec![
            Item { cost: 3, weight: 2 },
            Item { cost: 1, weight: 5 },
            Item { cost: 7, weight: 4 },
            Item { cost: 6, weight: 6 },
        ],
        knapsack_capacity: 11,
    };
    let data = data_source.data(&knapsack);
}
