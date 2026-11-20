use crate::scalars::DVar;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, SymWrap, VarType};
use alloc::format;
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
            let var_name = "x"; // TODO: var-name fix
            let var = DVar::create_d1::<$s0>(vard.ptr(), || $i0);
            assert_eq!(var.eval_to_string(), format!("{var_name}_{}", $i0));
        };
    }

    let three = 3.to_string();
    test_i0!(SetCatInt, 3);
    test_i0!(SetCatStr, &three);
    test_i0!(SetOrd, 3);
}
