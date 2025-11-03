use orx_math_model::*;

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

fn main() {
    let mcfp = Mcfp::new();

    println!("{:?}", mcfp.j());
    println!("{:?}", mcfp.i());
    println!("{:?}", mcfp.k());

    drop(mcfp);
}
