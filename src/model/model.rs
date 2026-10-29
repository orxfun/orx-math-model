use super::bag::{new_bag, Bag};
use crate::model::sets::Sets;

pub struct Model<'d> {
    sets: Sets<'d>,
}
