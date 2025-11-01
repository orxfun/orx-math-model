use orx_math_model::*;

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

fn main() {
    let knapsack = Knapsack::new();

    println!("{:?}", knapsack.i());

    drop(knapsack);
}
