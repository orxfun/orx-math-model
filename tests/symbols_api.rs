#![allow(non_snake_case, unused_variables)]

use orx_math_model::*;

// #[test]
// fn machine_scheduling() {
//     let m = Model::default();

//     let num_periods = m.int_par0();
//     let num_jobs = m.int_par0();

//     let T = m.range(1, num_periods);
//     let J = m.range(1, num_jobs);

//     let t = T.element();
//     let j = J.element();

//     let t = m.range(1, num_periods).element();
//     let j = m.range(1, num_jobs).element();

//     let tp = t + 1;
//     let tp = 1 + t;
//     let tp = t + t;
//     let tj = t + j;

//     let tp = m.range(1, num_periods + 1);
//     let tp = m.range(t, num_periods);

//     let N = m.set();
// }

// #[test]
// fn cat() {
//     let m = Model::default();
//     let J = m.set();

//     trait PossiblyResolvedSet {
//         type Idx;
//     }

//     struct ResolvedSet<C, T, I> {
//         elements: C,
//         get: Box<dyn Fn(&Self, I) -> &T>,
//         collect: Box<dyn Fn(&Self, &mut Vec<I>)>,
//     }

//     impl<C, T, I> PossiblyResolvedSet for ResolvedSet<C, T, I> {
//         type Idx = I;
//     }

//     type Jobs = ResolvedSet<Vec<Job>, Job, usize>;
//     struct Job {
//         time: i64,
//     }
//     let jobs = vec![Job { time: 3 }];
//     fn get<'a>(x: &'a Jobs, i: usize) -> &'a Job {
//         &x.elements[i]
//     }
//     let collect = |x: &Jobs, dst: &mut Vec<usize>| {
//         dst.clear();
//         dst.extend(0..x.elements.len());
//     };
//     let jobs = Jobs {
//         elements: jobs,
//         get: Box::new(get),
//         collect: Box::new(collect),
//     };

//     let jobs: Box<dyn PossiblyResolvedSet<Idx = usize>> = Box::new(jobs);

//     // parameters
// }
