use super::bag::{new_bag, Bag};
use crate::sets::{IntSetFun, SetFun};

pub struct Sets<'d> {
    int: Bag<SetFun<'d, usize>>,
    str: Bag<SetFun<'d, &'d str>>,
    int_set: IntSetFun<'d>,
}
