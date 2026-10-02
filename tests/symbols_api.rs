#![allow(non_snake_case, unused_variables)]

use orx_math_model::symbols::*;

#[test]
fn knapsack() {
    let m = Model::new("knapsack");

    let items = m.set();

    println!("{m:?}");
}

#[test]
fn mcfp() {
    let m = Model::new("mcfp");

    let nodes = m.set();

    println!("{m:?}");
}

#[test]
fn machine_scheduling() {
    let m = Model::new("machine_scheduling");

    let T = m.count();

    let jobs = m.set();
    let time = m.indexed_set(T);

    println!("{m:?}");
}
