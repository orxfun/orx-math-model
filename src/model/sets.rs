use super::bag::{new_bag, Bag};

pub struct Sets<'d> {
    int: &'d usize,
    str: &'d usize,
    idx: &'d usize,
}
