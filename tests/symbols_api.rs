// #![allow(non_snake_case, unused_variables)]
// use orx_math_model::*;

// // #[test]
// // fn knapsack() {
// //     let m = Model::new("knapsack");

// //     let item = m.cat_set();

// //     println!("{m:?}");
// // }

// // #[test]
// // fn mcfp() {
// //     let m = Model::new("mcfp");

// //     let node = m.cat_set();

// //     println!("{m:?}");
// // }

// #[test]
// fn machine_scheduling() {
//     let m = Model::default();

//     let T = m.int_par0();
//     let N = m.int_par0();

//     let t = m.range(1, T);
//     let j = m.range(1, N);
// }

// // #[test]
// // fn abc() {
// //     let parameter = {
// //         let original: Model<'static> = Model::default();
// //         original.int_par0()
// //     }; // original and its parameter storage are destroyed

// //     let another: Model<'static> = Model::default();
// //     let _range = another.range(1, parameter);
// // }
