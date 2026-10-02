use orx_math_model::symbols::*;

#[test]
fn knapsack() {
    let model = Model::new("knapsack");

    let items = model.set("items");

    println!("{model:?}");
}

#[test]
fn mcfp() {
    let model = Model::new("mcfp");

    let nodes = model.set("nodes");

    println!("{model:?}");
}

#[test]
fn machine_scheduling() {
    let model = Model::new("machine_scheduling");

    let T = model.count("T");

    let jobs = model.set("jobs");
    let time = model.indexed_set("time", T);

    println!("{model:?}");
}
