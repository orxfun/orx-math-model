#![allow(non_snake_case, unused_variables)]
use orx_math_model::*;

#[test]
fn machine_scheduling() {
    let m = Model::default();

    let num_periods = m.int_par0();
    let num_jobs = m.int_par0();

    let T = m.range(1, num_periods);
    let J = m.range(1, num_jobs);

    let t = T.element();
    let j = J.element();

    let t = m.range(1, num_periods).element();
    let j = m.range(1, num_jobs).element();
}
