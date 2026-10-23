use crate::symbols::{Idx, Set};
use crate::symbols_d1::{SetD1, SetD1Data};
use crate::Model;

// #[derive(Clone, Copy, PartialEq, Debug)]
// pub struct SetRef {
//     i: usize,
// }

// impl SetRef {
//     pub fn new<'m, 'd>(set: Set<'m, 'd>) -> Self {
//         let i = set.el.position_in(&set.m.set).expect("TODO: error msg");
//         Self { i }
//     }

//     pub fn set<'m, 'd>(self, m: &'m Model<'d>) -> Set<'m, 'd> {
//         Set::new(m, m.set.get(self.i).expect("TODO: error msg"))
//     }
// }

// #[derive(Clone, Copy, PartialEq, Debug)]
// pub struct SetD1Ref {
//     i: usize,
// }

// impl SetD1Ref {
//     pub fn new<'m, 'd>(set: SetD1<'m, 'd>) -> Self {
//         let i = set.el.position_in(&set.m.set_d1).expect("TODO: error msg");
//         Self { i }
//     }

//     pub fn set<'m, 'd>(self, m: &'m Model<'d>) -> SetD1<'m, 'd> {
//         SetD1::new(m, m.set_d1.get(self.i).expect("TODO: error msg"))
//     }
// }
