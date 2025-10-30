use crate::symbols::sets::set_gen::{Elements, SetGenZzz};
use alloc::vec::Vec;

macro_rules! define_set_vec_integer {
    ($set_name:ident, $int_type:ty) => {
        pub struct $set_name {
            values: Vec<usize>,
        }

        impl From<&[$int_type]> for $set_name {
            fn from(values: &[$int_type]) -> Self {
                let values = values.iter().map(|x| *x as usize).collect();
                Self { values }
            }
        }
    };
}

define_set_vec_integer!(SetVecU64, u64);
define_set_vec_integer!(SetVecU32, u32);
