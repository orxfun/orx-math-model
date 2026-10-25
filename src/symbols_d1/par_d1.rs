use crate::ptr::RefPtr;
use crate::symbols::{IndicesFun, Int, IntFun, Num, NumFun, PseudoInt};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

struct Fun<'d, T: Default>(Box<dyn Fn(usize) -> T + 'd>);
