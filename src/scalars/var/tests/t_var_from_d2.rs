use crate::scalars::num::{DNum, ValInt, ValNum};
use crate::scalars::DVar;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, SymWrap, VarType};
use alloc::boxed::Box;
use alloc::{format, string::ToString};

#[test]
fn var_from_var1() {
    macro_rules! test_i0_i1 {
        ($s0:ty, $i0:expr, $s1:ty, $i1:expr) => {
            // modeling
            let m = Model::new();
            let s0 = m.set::<$s0>();
            let s1 = m.set::<$s1>();
            let vard = m.var2(VarType::Binary, s0, s1);

            // running
            let var = DVar::create_d2::<$s0, $s1>(vard.data_ptr(), || ($i0, $i1));
            let exp = format!("{}_{}", $i0, $i1);
            assert_eq!(var.to_string(), exp);
        };
    }

    let three = 3.to_string();
    test_i0_i1!(SetCatInt, 3, SetCatStr, &three);
    test_i0_i1!(SetCatStr, &three, SetOrd, 3);
    test_i0_i1!(SetOrd, 3, SetCatInt, 3);
}
