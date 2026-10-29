use crate::model::sets::Sets;
use crate::sets::{Indices, Set};

pub struct Model<'d> {
    sets: Sets<'d>,
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
}
