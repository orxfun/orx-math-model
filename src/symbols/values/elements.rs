use alloc::boxed::Box;
use alloc::vec::Vec;

pub enum Elements {
    D0(Vec<usize>),
    D1(Box<dyn Fn(usize, &mut Vec<usize>)>),
}
