use crate::scalars::num::{DNum, ValInt, ValNum};
use crate::scalars::DVar;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, SymWrap, VarType};
use alloc::boxed::Box;
use alloc::string::ToString;

#[test]
fn var_from_var1() {
    macro_rules! test_i0 {
        ($s0:ty, $i0:expr) => {
            // modeling
            let m = Model::new();
            let s0 = m.set::<$s0>();
            let vard = m.var1(VarType::Binary, s0);

            // running
            let var = DVar::create_d1::<$s0>(vard.data_ptr(), Box::new(|| $i0));
            assert_eq!(var.indices_d1::<$s0>(), $i0);
        };
    }

    let three = 3.to_string();
    test_i0!(SetCatInt, 3);
    test_i0!(SetCatStr, &three);
    test_i0!(SetOrd, 3);
}
