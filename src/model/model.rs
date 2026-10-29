use crate::model::numbers::Numbers;
use crate::model::sets::Sets;
use crate::numbers::{Int, Num, ValFun};
use crate::sets::{Indices, Set};

pub struct Model<'d> {
    sets: Sets<'d>,
    numbers: Numbers<'d>,
}

impl<'d> Model<'d> {
    // sets

    pub fn set_int<'m>(&'m self) -> Set<'m, 'd, usize> {
        Set::new(self, self.sets.int())
    }

    pub fn set_str<'m>(&'m self) -> Set<'m, 'd, &'d str> {
        Set::new(self, self.sets.str())
    }

    pub fn indices<'m>(&'m self) -> Indices<'m, 'd> {
        Indices::new(self, self.sets.indices())
    }

    // numbers

    pub(crate) fn numbers_int<'m>(&'m self, fun: ValFun<'d, i64>) -> Int<'m, 'd> {
        Int::new(self, self.numbers.int(fun))
    }

    pub(crate) fn numbers_num<'m>(&'m self, fun: ValFun<'d, f64>) -> Num<'m, 'd> {
        Num::new(self, self.numbers.num(fun))
    }
}
